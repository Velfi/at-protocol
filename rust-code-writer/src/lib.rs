mod dependency;
mod metadata;
mod module;
mod tree;

pub use metadata::CrateMetadata;
pub use module::{Module, Parent, Visibility};
use std::{
    collections::HashSet,
    fmt::{Display, Write},
    path::Path,
};
use tracing::debug;
use tree::Modules;

pub struct Crate {
    metadata: CrateMetadata,
    modules: Modules,
    pending_writes: Vec<(Module, String)>,
}

impl Crate {
    pub fn new(metadata: CrateMetadata) -> Self {
        Self {
            metadata,
            modules: Modules::new(),
            pending_writes: Vec::new(),
        }
    }

    fn add_module_to_tree(&mut self, module: Module) {
        self.modules.add_module_to_tree(module);
    }

    pub fn write(&mut self, module: Module, content: impl Display) {
        self.pending_writes
            .push((module.clone(), content.to_string()));
        self.add_module_to_tree(module);
    }

    pub fn writer(&mut self, module: Module) -> impl Write + '_ {
        self.add_module_to_tree(module.clone());
        self.get_or_create_writer(module)
    }

    fn get_or_create_writer(&mut self, module: Module) -> impl Write + '_ {
        let v = String::new();
        self.pending_writes.push((module, v));

        &mut self.pending_writes.last_mut().unwrap().1
    }

    pub fn finalize(mut self, directory_to_write_to: &Path) -> Result<(), Error> {
        let crate_dir = directory_to_write_to.join(&self.metadata.name);
        debug!("creating crate directory: {}", crate_dir.display());
        std::fs::create_dir_all(&crate_dir).map_err(|source| Error::Finalization { source })?;

        self.merge_pending_writes();
        self.write_cargo_toml(&crate_dir)
            .map_err(|source| Error::Finalization { source })?;
        self.create_module_directories(&crate_dir)
            .map_err(|source| Error::Finalization { source })?;
        self.write_module_files(&crate_dir)
            .map_err(|source| Error::Finalization { source })?;

        Ok(())
    }

    fn merge_pending_writes(&mut self) {
        self.pending_writes
            .sort_by(|a, b| a.0.to_module_path().cmp(&b.0.to_module_path()));
        let unmerged_writes = std::mem::take(&mut self.pending_writes);

        self.pending_writes =
            unmerged_writes
                .into_iter()
                .fold(Vec::new(), |mut acc, (module, contents)| {
                    if let Some((current_module, current_contents)) = acc.last_mut() {
                        // If the two modules have the same path, merge them and their contents
                        if *current_module == module {
                            current_module.merge(&module);
                            current_contents.push_str(&contents);
                            return acc;
                        }
                    }

                    // Otherwise, add the module and its contents to the list
                    acc.push((module, contents));
                    acc
                });
    }

    fn write_cargo_toml(&self, directory_to_write_to: &Path) -> Result<(), std::io::Error> {
        use std::io::Write;

        let cargo_toml_path = directory_to_write_to.join("Cargo.toml");
        debug!("writing Cargo.toml to {}", cargo_toml_path.display());
        let mut cargo_toml = std::fs::File::create(cargo_toml_path)?;

        writeln!(cargo_toml, "{}", self.metadata)?;
        writeln!(cargo_toml)?;

        writeln!(cargo_toml, "{}", self.modules)?;
        writeln!(cargo_toml)?;

        Ok(())
    }

    fn create_module_directories(
        &self,
        directory_to_write_to: &Path,
    ) -> Result<(), std::io::Error> {
        let paths: HashSet<_> = self
            .modules
            .module_file_paths()
            .into_iter()
            .map(|p| directory_to_write_to.join(p))
            .map(|mut p| {
                // Pop the file name off the end because we only want the directory
                p.pop();
                p
            })
            .collect();
        for dir in paths {
            debug!("creating module directory: {}", dir.display());
            std::fs::create_dir_all(dir)?;
        }

        Ok(())
    }

    fn write_module_files(&self, directory_to_write_to: &Path) -> Result<(), std::io::Error> {
        for (module, contents) in self.pending_writes.iter() {
            let file_path = directory_to_write_to.join(module.to_file_path());
            debug!("writing to file: {}", file_path.display());
            std::fs::write(file_path, contents)?;
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("an unknown error occurred: {message}")]
    Unknown { message: String },
    #[error("finalization of the crate failed: {source}")]
    Finalization { source: std::io::Error },
}
