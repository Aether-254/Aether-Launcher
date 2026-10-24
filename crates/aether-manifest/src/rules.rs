use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// 规则要匹配的运行环境。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Environment {
    /// 清单里的系统名：`windows`、`linux`、`osx`
    pub os_name: String,
    /// 清单里的架构名，例如 `x86`
    pub os_arch: String,
    /// 启用的特性，例如 `is_demo_user`、`has_custom_resolution`
    pub features: BTreeSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleAction {
    Allow,
    Disallow,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OsRule {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub arch: Option<String>,
    /// 正则表达式。目前不评估，带 `version` 的系统规则一律视为不匹配。
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rule {
    pub action: RuleAction,
    #[serde(default)]
    pub os: Option<OsRule>,
    #[serde(default)]
    pub features: BTreeMap<String, bool>,
}

impl Rule {
    pub fn matches(&self, environment: &Environment) -> bool {
        let os_matches = match &self.os {
            Some(os) => os.matches(environment),
            None => true,
        };
        os_matches && features_match(&self.features, environment)
    }
}

impl OsRule {
    fn matches(&self, environment: &Environment) -> bool {
        if self.version.is_some() {
            return false;
        }
        option_matches(self.name.as_deref(), &environment.os_name)
            && option_matches(self.arch.as_deref(), &environment.os_arch)
    }
}

fn option_matches(expected: Option<&str>, actual: &str) -> bool {
    expected.is_none_or(|value| value == actual)
}

fn features_match(required: &BTreeMap<String, bool>, environment: &Environment) -> bool {
    required
        .iter()
        .all(|(name, expected)| environment.features.contains(name) == *expected)
}

/// 没有规则时允许；有规则时，最后一条匹配的规则决定结果，一条都不匹配则不允许。
pub fn rules_allow(rules: &[Rule], environment: &Environment) -> bool {
    if rules.is_empty() {
        return true;
    }
    let mut allowed = false;
    for rule in rules {
        if rule.matches(environment) {
            allowed = rule.action == RuleAction::Allow;
        }
    }
    allowed
}

#[cfg(test)]
mod tests {
    use super::{Environment, Rule, rules_allow};

    const ALLOW_ALL_EXCEPT_OSX: &str = r#"
    [
      {"action": "allow"},
      {"action": "disallow", "os": {"name": "osx"}}
    ]
    "#;

    const ALLOW_ONLY_WINDOWS: &str = r#"
    [{"action": "allow", "os": {"name": "windows"}}]
    "#;

    const ALLOW_DEMO_ONLY: &str = r#"
    [{"action": "allow", "features": {"is_demo_user": true}}]
    "#;

    const ALLOW_WINDOWS_X86: &str = r#"
    [{"action": "allow", "os": {"name": "windows", "arch": "x86"}}]
    "#;

    const DISALLOW_BY_VERSION: &str = r#"
    [
      {"action": "allow"},
      {"action": "disallow", "os": {"version": "^10\\."}}
    ]
    "#;

    fn environment(os_name: &str, features: &[&str]) -> Environment {
        let mut environment = Environment {
            os_name: os_name.to_owned(),
            os_arch: "x86_64".to_owned(),
            ..Environment::default()
        };
        for feature in features {
            environment.features.insert((*feature).to_owned());
        }
        environment
    }

    fn rules(json: &str) -> Vec<Rule> {
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn no_rules_means_allowed() {
        assert!(rules_allow(&[], &environment("windows", &[])));
    }

    #[test]
    fn disallow_rule_after_allow_all_excludes_that_system() {
        let rules = rules(ALLOW_ALL_EXCEPT_OSX);
        assert!(rules_allow(&rules, &environment("windows", &[])));
        assert!(!rules_allow(&rules, &environment("osx", &[])));
    }

    #[test]
    fn allow_rule_for_one_system_excludes_the_others() {
        let rules = rules(ALLOW_ONLY_WINDOWS);
        assert!(rules_allow(&rules, &environment("windows", &[])));
        assert!(!rules_allow(&rules, &environment("linux", &[])));
    }

    #[test]
    fn feature_rules_need_the_feature_to_be_enabled() {
        let rules = rules(ALLOW_DEMO_ONLY);
        let with_demo = environment("linux", &["is_demo_user"]);
        let without_demo = environment("linux", &[]);
        assert!(rules_allow(&rules, &with_demo));
        assert!(!rules_allow(&rules, &without_demo));
    }

    #[test]
    fn architecture_must_match_when_given() {
        let rules = rules(ALLOW_WINDOWS_X86);
        assert!(!rules_allow(&rules, &environment("windows", &[])));
    }

    #[test]
    fn system_rules_with_a_version_never_match() {
        let rules = rules(DISALLOW_BY_VERSION);
        assert!(rules_allow(&rules, &environment("windows", &[])));
    }
}
