use serde::{Deserialize, Serialize};

use crate::rules::{Environment, Rule, rules_allow};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArgumentValue {
    One(String),
    Many(Vec<String>),
}

/// 清单里的一个参数：普通字符串，或者带规则的条件参数。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Argument {
    Plain(String),
    Conditional {
        #[serde(default)]
        rules: Vec<Rule>,
        value: ArgumentValue,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Arguments {
    #[serde(default)]
    pub game: Vec<Argument>,
    #[serde(default)]
    pub jvm: Vec<Argument>,
}

/// 按环境展开参数列表，保持清单里的顺序。`${...}` 占位符原样保留。
pub fn expand_arguments(arguments: &[Argument], environment: &Environment) -> Vec<String> {
    let mut expanded = Vec::new();
    for argument in arguments {
        match argument {
            Argument::Plain(value) => expanded.push(value.clone()),
            Argument::Conditional { rules, value } => {
                if !rules_allow(rules, environment) {
                    continue;
                }
                match value {
                    ArgumentValue::One(value) => expanded.push(value.clone()),
                    ArgumentValue::Many(values) => expanded.extend(values.iter().cloned()),
                }
            }
        }
    }
    expanded
}

#[cfg(test)]
mod tests {
    use super::{Arguments, expand_arguments};
    use crate::rules::Environment;

    const ARGUMENTS: &str = r#"
    {
      "game": [
        "--username", "${auth_player_name}",
        {"rules": [{"action": "allow", "features": {"is_demo_user": true}}], "value": "--demo"}
      ],
      "jvm": [
        {"rules": [{"action": "allow", "os": {"name": "osx"}}], "value": ["-XstartOnFirstThread"]},
        "-cp", "${classpath}"
      ]
    }
    "#;

    fn environment(os_name: &str, feature: Option<&str>) -> Environment {
        let mut environment = Environment {
            os_name: os_name.to_owned(),
            ..Environment::default()
        };
        if let Some(feature) = feature {
            environment.features.insert(feature.to_owned());
        }
        environment
    }

    #[test]
    fn plain_arguments_are_kept_and_conditional_ones_follow_the_rules() {
        let arguments: Arguments = serde_json::from_str(ARGUMENTS).unwrap();

        let plain = environment("windows", None);
        let game = expand_arguments(&arguments.game, &plain);
        assert_eq!(game, ["--username", "${auth_player_name}"]);

        let with_demo = environment("windows", Some("is_demo_user"));
        let demo = expand_arguments(&arguments.game, &with_demo);
        assert_eq!(demo, ["--username", "${auth_player_name}", "--demo"]);
    }

    #[test]
    fn array_values_expand_in_place() {
        let arguments: Arguments = serde_json::from_str(ARGUMENTS).unwrap();

        let osx_environment = environment("osx", None);
        let osx = expand_arguments(&arguments.jvm, &osx_environment);
        assert_eq!(osx, ["-XstartOnFirstThread", "-cp", "${classpath}"]);

        let linux_environment = environment("linux", None);
        let linux = expand_arguments(&arguments.jvm, &linux_environment);
        assert_eq!(linux, ["-cp", "${classpath}"]);
    }
}
