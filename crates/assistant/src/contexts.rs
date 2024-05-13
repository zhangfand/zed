use fs::Fs;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use util::paths::CONTEXTS_DIR;

#[derive(Serialize, Deserialize, Debug)]
struct UserContext {
    id: Option<String>,
    version: String,
    title: String,
    author: String,
    languages: Option<Vec<String>>,
    content: String,
}

impl UserContext {
    async fn list(fs: Arc<dyn Fs>) -> anyhow::Result<Vec<UserContext>> {
        let dir_string = CONTEXTS_DIR.to_string_lossy();

        println!("Creating/verifying directory at {:?}", dir_string);
        fs.create_dir(&CONTEXTS_DIR).await?;

        println!("Reading directory entries in {:?}", dir_string);
        let mut paths = fs.read_dir(&CONTEXTS_DIR).await?;
        let mut contexts = Vec::new();

        while let Some(path_result) = paths.next().await {
            let path = match path_result {
                Ok(p) => p,
                Err(e) => {
                    println!("Error reading path from {:?}: {:?}", dir_string, e);
                    continue; // Skip this iteration on error
                }
            };

            // Correct approach: Directly check for JSON files without attempting to read them as directories.
            if path.extension() == Some(std::ffi::OsStr::new("json")) {
                println!("Attempting to load and deserialize JSON from {:?}", path);
                match fs.load(&path).await {
                    Ok(content) => match serde_json::from_str::<UserContext>(&content) {
                        Ok(context) => contexts.push(context),
                        Err(e) => eprintln!("Failed to deserialize {}: {}", path.display(), e),
                    },
                    Err(e) => eprintln!("Failed to load file {}: {}", path.display(), e),
                }
            }
        }

        Ok(contexts)
    }
}

pub fn log_contexts(fs: Arc<dyn Fs>) {
    futures::executor::block_on(async {
        match UserContext::list(fs).await {
            Ok(contexts) => {
                for context in contexts {
                    println!("{:#?}", context);
                }
            }
            Err(e) => eprintln!("Failed to list contexts: {}", e),
        }
    });
}
