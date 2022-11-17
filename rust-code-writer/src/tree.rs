use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use crate::dependency::CargoDependency;
use crate::Module;

pub struct Modules {
    modules: HashMap<String, Module>,
}

impl Modules {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn add_module_to_tree(&mut self, module: Module) {
        let path = module.to_module_path();

        if self.modules.contains_key(&path) {
            // TODO Maybe we should merge module deps here instead of when writing the modules?
            return;
        } else {
            self.modules.insert(path, module);
        }
    }

    pub fn module_file_paths(&self) -> Vec<PathBuf> {
        self.modules.values().map(|m| m.to_file_path()).collect()
    }

    fn dependencies(&self) -> Vec<CargoDependency> {
        self.modules
            .values()
            .map(|module| module.dependencies.clone())
            .flatten()
            .collect()
    }
}

impl fmt::Display for Modules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dependencies = self.dependencies();

        if dependencies.is_empty() {
            return Ok(());
        }

        dependencies.sort_by(|a, b| a.name.cmp(&b.name));

        writeln!(f, "[dependencies]")?;

        for dependency in dependencies {
            writeln!(f, "{}", dependency)?;
        }

        Ok(())
    }
}
