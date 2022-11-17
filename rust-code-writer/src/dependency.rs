use std::fmt;

#[derive(Debug, Clone)]
pub struct CargoDependency {
    pub name: String,
    pub version: String,
    pub features: Vec<String>,
}

impl CargoDependency {
    pub fn new(name: String, version: String, features: Vec<String>) -> Self {
        Self {
            name,
            version,
            features,
        }
    }
}

impl fmt::Display for CargoDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.features.is_empty() {
            writeln!(f, "{} = \"{}\"", self.name, self.version)
        } else {
            writeln!(
                f,
                "{} = {{ version = \"{}\", features = [{}] }}",
                self.name,
                self.version,
                self.features
                    .iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CargoDependency;

    #[test]
    fn test_print_cargo_dependency_with_no_features() {
        let dependency =
            CargoDependency::new("test-crate".to_string(), "0.1.0".to_string(), vec![]);
        assert_eq!("test-crate = \"0.1.0\"\n", dependency.to_string(),);
    }

    #[test]
    fn test_print_cargo_dependency_with_features() {
        let dependency = CargoDependency::new(
            "test-crate".to_string(),
            "0.1.0".to_string(),
            vec!["feature1".to_string(), "feature2".to_string()],
        );
        assert_eq!(
            "test-crate = { version = \"0.1.0\", features = [\"feature1\", \"feature2\"] }\n",
            dependency.to_string(),
        );
    }
}
