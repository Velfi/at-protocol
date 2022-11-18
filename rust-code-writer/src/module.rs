use crate::dependency::CargoDependency;
use std::fmt;
use std::{borrow::Cow, path::PathBuf};

#[derive(Clone)]
pub enum Visibility {
    Public,
    Private,
    PubCrate,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Visibility::Public => write!(f, "pub "),
            Visibility::Private => write!(f, ""),
            Visibility::PubCrate => write!(f, "pub(crate) "),
        }
    }
}

#[derive(Clone)]
pub struct Module {
    pub name: Cow<'static, str>,
    pub documentation: Cow<'static, str>,
    pub parent: Parent,
    pub dependencies: Vec<CargoDependency>,
    pub visibility: Visibility,
}

impl Module {
    pub fn to_module_path(&self) -> String {
        if matches!(self.parent, Parent::Lib) {
            format!("crate::{}", self.name)
        } else {
            let mut path = String::new();
            path.push_str("crate");
            for part in self.to_path() {
                path.push_str("::");
                path.push_str(part);
            }

            path
        }
    }

    pub fn to_file_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        for part in self.to_path() {
            path.push(part.as_ref());
        }
        path.set_extension("rs");

        path
    }

    pub fn merge(&mut self, other: &Module) {
        self.dependencies.extend(other.dependencies.clone());
    }

    fn to_path(&self) -> impl Iterator<Item = &Cow<'_, str>> {
        let mut path = Vec::new();
        let mut current_module = self;
        loop {
            path.push(&current_module.name);
            match &current_module.parent {
                Parent::Lib => break,
                Parent::Module(module) => {
                    current_module = module;
                }
            }
        }

        path.into_iter().rev()
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.to_module_path() == other.to_module_path()
    }
}

impl Eq for Module {}

#[derive(Clone)]
pub enum Parent {
    Lib,
    Module(Box<Module>),
}

#[cfg(test)]
mod tests {
    use super::{Module, Parent, Visibility};
    use std::path::PathBuf;

    fn create_test_module() -> Module {
        let top_level_module = Module {
            name: "top_level".into(),
            documentation: "The top module".into(),
            parent: Parent::Lib,
            dependencies: Vec::new(),
            visibility: Visibility::Public,
        };

        let middle_level_module = Module {
            name: "middle_level".into(),
            documentation: "The middle module".into(),
            parent: Parent::Module(Box::new(top_level_module.clone())),
            dependencies: Vec::new(),
            visibility: Visibility::Public,
        };

        Module {
            name: "bottom_level".into(),
            documentation: "The bottom module".into(),
            parent: Parent::Module(Box::new(middle_level_module)),
            dependencies: Vec::new(),
            visibility: Visibility::Public,
        }
    }

    #[test]
    fn test_to_module_path() {
        let test_module = create_test_module();
        let module_path = test_module.to_module_path();

        assert_eq!("crate::top_level::middle_level::bottom_level", module_path);
    }

    #[test]
    fn test_to_file_path() {
        let test_module = create_test_module();
        let file_path = test_module.to_file_path();
        let expected_path = PathBuf::from("top_level/middle_level/bottom_level.rs");

        assert_eq!(expected_path, file_path);
    }
}
