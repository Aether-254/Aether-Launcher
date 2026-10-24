use std::cmp::Ordering;

use crate::arguments::Arguments;
use crate::{Library, VersionManifest};

impl VersionManifest {
    /// 把这个清单（子版本，例如 Fabric 或 Forge）叠加到已经解析好的父版本上。
    ///
    /// - 库：子版本在前、父版本在后。坐标（group、artifact、classifier）和规则都相同的库只留一个，
    ///   版本更高的赢，版本相同留先出现的（子版本的）。规则不同的同名库都保留。
    /// - 参数：父版本在前、子版本在后。
    /// - 其余字段：子版本有就用子版本的，否则用父版本的。
    ///
    /// 多级继承由调用方从最顶层的父版本开始逐级合并。
    pub fn merge_with_parent(&self, parent: &VersionManifest) -> VersionManifest {
        let mut libraries = self.libraries.clone();
        libraries.extend(parent.libraries.iter().cloned());

        VersionManifest {
            id: self.id.clone(),
            inherits_from: None,
            main_class: child_or_parent(&self.main_class, &parent.main_class),
            assets: child_or_parent(&self.assets, &parent.assets),
            asset_index: child_or_parent(&self.asset_index, &parent.asset_index),
            downloads: child_or_parent(&self.downloads, &parent.downloads),
            java_version: child_or_parent(&self.java_version, &parent.java_version),
            libraries: unique_libraries(libraries),
            arguments: merge_arguments(parent.arguments.as_ref(), self.arguments.as_ref()),
            minecraft_arguments: child_or_parent(
                &self.minecraft_arguments,
                &parent.minecraft_arguments,
            ),
        }
    }
}

fn child_or_parent<T: Clone>(child: &Option<T>, parent: &Option<T>) -> Option<T> {
    child.as_ref().or(parent.as_ref()).cloned()
}

fn merge_arguments(parent: Option<&Arguments>, child: Option<&Arguments>) -> Option<Arguments> {
    match (parent, child) {
        (Some(parent), Some(child)) => {
            let mut game = parent.game.clone();
            game.extend(child.game.iter().cloned());
            let mut jvm = parent.jvm.clone();
            jvm.extend(child.jvm.iter().cloned());
            Some(Arguments { game, jvm })
        }
        (Some(only), None) | (None, Some(only)) => Some(only.clone()),
        (None, None) => None,
    }
}

/// `group:artifact:version[:classifier][@ext]`
struct Coordinate<'a> {
    group: &'a str,
    artifact: &'a str,
    version: &'a str,
    classifier: Option<&'a str>,
}

impl Coordinate<'_> {
    fn same_slot(&self, other: &Coordinate<'_>) -> bool {
        self.group == other.group
            && self.artifact == other.artifact
            && self.classifier == other.classifier
    }
}

fn parse_coordinate(name: &str) -> Option<Coordinate<'_>> {
    let name = name.split('@').next().unwrap_or(name);
    let mut parts = name.split(':');
    let group = parts.next()?;
    let artifact = parts.next()?;
    let version = parts.next()?;
    let classifier = parts.next();
    if group.is_empty() || artifact.is_empty() || version.is_empty() {
        return None;
    }
    Some(Coordinate {
        group,
        artifact,
        version,
        classifier,
    })
}

fn unique_libraries(libraries: Vec<Library>) -> Vec<Library> {
    let mut unique: Vec<Library> = Vec::new();
    for library in libraries {
        let Some(candidate) = parse_coordinate(&library.name) else {
            unique.push(library);
            continue;
        };

        let existing = unique.iter().position(|other| {
            parse_coordinate(&other.name)
                .is_some_and(|key| key.same_slot(&candidate) && other.rules == library.rules)
        });

        match existing {
            None => unique.push(library),
            Some(index) => {
                let current = parse_coordinate(&unique[index].name)
                    .map(|key| key.version)
                    .unwrap_or_default();
                if compare_versions(candidate.version, current) == Ordering::Greater {
                    unique[index] = library;
                }
            }
        }
    }
    unique
}

#[derive(Debug, PartialEq, Eq)]
enum Part<'a> {
    Number(u64),
    Text(&'a str),
}

fn parts(version: &str) -> Vec<Part<'_>> {
    version
        .split(['.', '-', '_', '+'])
        .filter(|part| !part.is_empty())
        .map(|part| match part.parse::<u64>() {
            Ok(number) => Part::Number(number),
            Err(_) => Part::Text(part),
        })
        .collect()
}

/// 按段比较版本：数字按数值比，数字大于文字限定词（`1.0` 大于 `1.0-beta`），
/// 多出来的数字段让版本更高（`1.0.1` 大于 `1.0`），多出来的 `0` 忽略（`1.0` 等于 `1.0.0`）。
fn compare_versions(left: &str, right: &str) -> Ordering {
    let left = parts(left);
    let right = parts(right);
    for index in 0..left.len().max(right.len()) {
        let ordering = match (left.get(index), right.get(index)) {
            (Some(Part::Number(a)), Some(Part::Number(b))) => a.cmp(b),
            (Some(Part::Number(_)), Some(Part::Text(_))) => Ordering::Greater,
            (Some(Part::Text(_)), Some(Part::Number(_))) => Ordering::Less,
            (Some(Part::Text(a)), Some(Part::Text(b))) => {
                a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase())
            }
            (Some(extra), None) => extra_part_ordering(extra),
            (None, Some(extra)) => extra_part_ordering(extra).reverse(),
            (None, None) => Ordering::Equal,
        };
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    Ordering::Equal
}

fn extra_part_ordering(extra: &Part<'_>) -> Ordering {
    match extra {
        Part::Number(0) => Ordering::Equal,
        Part::Number(_) => Ordering::Greater,
        Part::Text(_) => Ordering::Less,
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::arguments::expand_arguments;
    use crate::rules::Environment;
    use crate::{VersionManifest, parse_version_manifest};

    use super::compare_versions;

    const PARENT: &str = r#"
    {
      "id": "1.21.1",
      "mainClass": "net.minecraft.client.main.Main",
      "assets": "17",
      "javaVersion": {"component": "java-runtime-delta", "majorVersion": 21},
      "downloads": {
        "client": {
          "sha1": "2222222222222222222222222222222222222222",
          "size": 1,
          "url": "https://example.invalid/client.jar"
        }
      },
      "libraries": [
        {"name": "com.example:common:1.0"},
        {"name": "org.ow2.asm:asm:9.6"},
        {
          "name": "com.example:native:1.0:natives-windows",
          "rules": [{"action": "allow", "os": {"name": "windows"}}]
        }
      ],
      "arguments": {
        "game": ["--version", "${version_name}"],
        "jvm": ["-cp", "${classpath}"]
      }
    }
    "#;

    const CHILD: &str = r#"
    {
      "id": "fabric-loader-0.16.0-1.21.1",
      "inheritsFrom": "1.21.1",
      "mainClass": "net.fabricmc.loader.impl.launch.knot.KnotClient",
      "libraries": [
        {"name": "net.fabricmc:fabric-loader:0.16.0"},
        {"name": "org.ow2.asm:asm:9.7"},
        {"name": "com.example:common:0.9"}
      ],
      "arguments": {"jvm": ["-DFabricMcEmu=true"]}
    }
    "#;

    fn manifest(json: &str) -> VersionManifest {
        parse_version_manifest(json).unwrap()
    }

    fn names(manifest: &VersionManifest) -> Vec<String> {
        let mut names = Vec::new();
        for library in &manifest.libraries {
            names.push(library.name.clone());
        }
        names
    }

    #[test]
    fn child_fields_win_and_missing_ones_come_from_the_parent() {
        let merged = manifest(CHILD).merge_with_parent(&manifest(PARENT));

        assert_eq!(merged.id, "fabric-loader-0.16.0-1.21.1");
        assert_eq!(merged.inherits_from, None);
        let main_class = merged.main_class.as_deref();
        assert_eq!(
            main_class,
            Some("net.fabricmc.loader.impl.launch.knot.KnotClient")
        );
        assert_eq!(merged.assets.as_deref(), Some("17"));
        assert_eq!(merged.java_version.unwrap().major_version, 21);
        assert!(merged.downloads.unwrap().client.is_some());
    }

    #[test]
    fn libraries_put_the_child_first_and_keep_the_newest_version() {
        let merged = manifest(CHILD).merge_with_parent(&manifest(PARENT));

        let expected = [
            "net.fabricmc:fabric-loader:0.16.0",
            "org.ow2.asm:asm:9.7",
            "com.example:common:1.0",
            "com.example:native:1.0:natives-windows",
        ];
        assert_eq!(names(&merged), expected);
    }

    #[test]
    fn arguments_put_the_parent_first() {
        let merged = manifest(CHILD).merge_with_parent(&manifest(PARENT));
        let arguments = merged.arguments.unwrap();
        let environment = Environment::default();

        let game = expand_arguments(&arguments.game, &environment);
        assert_eq!(game, ["--version", "${version_name}"]);
        let jvm = expand_arguments(&arguments.jvm, &environment);
        assert_eq!(jvm, ["-cp", "${classpath}", "-DFabricMcEmu=true"]);
    }

    #[test]
    fn libraries_with_different_rules_are_both_kept() {
        let child = manifest(
            r#"{"id":"c","libraries":[
              {"name":"a:b:2","rules":[{"action":"allow","os":{"name":"osx"}}]}]}"#,
        );
        let parent = manifest(
            r#"{"id":"p","libraries":[
              {"name":"a:b:1","rules":[{"action":"allow","os":{"name":"windows"}}]}]}"#,
        );

        let merged = child.merge_with_parent(&parent);

        assert_eq!(names(&merged), ["a:b:2", "a:b:1"]);
    }

    #[test]
    fn equal_versions_keep_the_childs_entry() {
        let child =
            manifest(r#"{"id":"c","libraries":[{"name":"a:b:1","url":"https://child.invalid/"}]}"#);
        let parent = manifest(
            r#"{"id":"p","libraries":[{"name":"a:b:1","url":"https://parent.invalid/"}]}"#,
        );

        let merged = child.merge_with_parent(&parent);

        assert_eq!(merged.libraries.len(), 1);
        assert_eq!(
            merged.libraries[0].url.as_deref(),
            Some("https://child.invalid/")
        );
    }

    #[test]
    fn names_that_are_not_maven_coordinates_are_never_merged() {
        let child = manifest(r#"{"id":"c","libraries":[{"name":"odd"}]}"#);
        let parent = manifest(r#"{"id":"p","libraries":[{"name":"odd"}]}"#);

        let merged = child.merge_with_parent(&parent);

        assert_eq!(names(&merged), ["odd", "odd"]);
    }

    #[test]
    fn versions_compare_by_segment() {
        let cases = [
            ("1.10", "1.9", Ordering::Greater),
            ("9.4.2", "9.4.10", Ordering::Less),
            ("1.0", "1.0.0", Ordering::Equal),
            ("1.0.1", "1.0", Ordering::Greater),
            ("1.0-beta", "1.0", Ordering::Less),
            ("1.0", "1.0-beta", Ordering::Greater),
            ("1.0-beta", "1.0-rc", Ordering::Less),
            ("0.16.0", "0.15.11", Ordering::Greater),
            ("9.7", "9.6", Ordering::Greater),
        ];
        for (left, right, expected) in cases {
            assert_eq!(compare_versions(left, right), expected, "{left} vs {right}");
        }
    }
}
