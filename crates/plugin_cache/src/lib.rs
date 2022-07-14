use std::{
    io::Read,
    ops::Not,
    path::{Path, PathBuf},
};

pub struct PluginCache {
    root: PathBuf,
}

pub enum Error {
    FileSystem,
    PluginExists,
}

impl PluginCache {
    /// Initializes an plugin cache,
    /// Creating a new one if one does not yet exist
    pub fn new(root: &Path) -> Result<Self, Error> {
        if root.exists().not() {
            std::fs::create_dir_all(root).map_err(|_| Error::FileSystem)?;
        }

        Ok(PluginCache {
            root: root.to_path_buf(),
        })
    }

    pub fn add_plugin(&self, name: &str) -> Result<(), Error> {
        let path = self.root.join(name);
        if path.exists() {
            return Err(Error::PluginExists);
        }

        std::fs::create_dir(path).map_err(|_| Error::FileSystem)?;
        std::fs::File::create(path.join("versions.txt")).map_err(|_| Error::FileSystem)?;
        Ok(())
    }

    pub fn add_version(&self, name: &str, compiled_wasm: &Path) -> Result<(), Error> {
        let path = self.root.join(name);
        if path.exists().not() {
            self.add_plugin(name)?;
        }

        let bytes = std::fs::read(compiled_wasm).map_err(|_| Error::FileSystem)?;
        let hash = blake3::hash(&bytes);

        todo!()
    }
}
