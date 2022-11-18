use std::fmt;

pub struct CrateMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub authors: Vec<String>,
}

impl fmt::Display for CrateMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[package]")?;
        writeln!(f, "name = \"{}\"", self.name)?;
        writeln!(f, "version = \"{}\"", self.version)?;
        writeln!(f, "edition = \"2021\"")?;
        writeln!(f, "description = \"{}\"", self.description)?;
        writeln!(f, "authors = [")?;
        for author in &self.authors {
            writeln!(f, "    \"{}\",", author)?;
        }
        writeln!(f, "]")?;
        writeln!(f)?;

        Ok(())
    }
}
