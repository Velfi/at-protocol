use crate::dependency::CargoDependency;
use crate::Module;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

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

        if let Entry::Vacant(e) = self.modules.entry(path) {
            e.insert(module);
        }
        // TODO Maybe we should merge module deps here instead of when writing the modules?
    }

    pub fn module_file_paths(&self) -> Vec<PathBuf> {
        self.modules.values().map(|m| m.to_file_path()).collect()
    }

    fn dependencies(&self) -> Vec<CargoDependency> {
        self.modules
            .values()
            .flat_map(|module| module.dependencies.clone())
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
