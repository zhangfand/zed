use fs::Fs;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use util::paths::PROMPTS_DIR;

pub struct PromptLibrary {
    prompts: HashMap<String, UserPrompt>,
    active_prompts: Vec<UserPrompt>,
}

impl PromptLibrary {
    pub fn new() -> Self {
        Self {
            prompts: HashMap::new(),
            active_prompts: Vec::new(),
        }
    }

    pub fn load_prompts(&mut self, fs: Arc<dyn Fs>) -> anyhow::Result<()> {
        let prompts = futures::executor::block_on(UserPrompt::list(fs))?;
        for prompt in prompts {
            let id = uuid::Uuid::new_v4().to_string();
            self.prompts.insert(id, prompt);
        }
        Ok(())
    }

    pub fn activate_prompt(&mut self, prompt_id: String) -> anyhow::Result<()> {
        let prompt = self
            .prompts
            .get(&prompt_id)
            .ok_or_else(|| anyhow::anyhow!("Prompt not found"))?;
        self.active_prompts.push(prompt.clone());
        Ok(())
    }

    pub fn activate_prompts(&mut self, prompt_ids: Vec<String>) -> anyhow::Result<()> {
        for id in prompt_ids {
            self.activate_prompt(id)?;
        }
        Ok(())
    }

    pub fn deactivate_prompt(&mut self, prompt_id: String) -> anyhow::Result<()> {
        let index = self
            .active_prompts
            .iter()
            .position(|p| p.metadata.title == prompt_id)
            .ok_or_else(|| anyhow::anyhow!("Prompt not found"))?;
        self.active_prompts.remove(index);
        Ok(())
    }

    pub fn join_active_prompts(&self) -> String {
        self.active_prompts
            .iter()
            .map(|p| p.content.clone())
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    }
}

pub fn prompt_library_example(fs: Arc<dyn Fs>) {
    let mut library = PromptLibrary::new();
    match library.load_prompts(fs) {
        Ok(_) => {
            let prompt_ids: Vec<String> = library.prompts.keys().cloned().collect();

            library
                .activate_prompts(prompt_ids)
                .expect("Failed to activate prompts");

            println!("{}", library.join_active_prompts());
        }
        Err(e) => eprintln!("Failed to load prompts: {}", e),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptMetadata {
    title: String,
    author: String,
    #[serde(default)]
    languages: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPrompt {
    metadata: PromptMetadata,
    content: String,
}

impl UserPrompt {
    fn parse_metadata(content: &str) -> anyhow::Result<(PromptMetadata, String)> {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() >= 3 {
            let frontmatter_str = parts[1].trim();
            let metadata: PromptMetadata = serde_yml::from_str(frontmatter_str)
                .map_err(|e| anyhow::anyhow!("Failed to parse front matter: {}", e))?;

            let content_body = parts.get(2).map_or("", |s| *s).trim();

            Ok((metadata, content_body.to_string()))
        } else {
            Err(anyhow::anyhow!("Invalid or missing front matter"))
        }
    }

    async fn list(fs: Arc<dyn Fs>) -> anyhow::Result<Vec<Self>> {
        fs.create_dir(&PROMPTS_DIR).await?;

        let mut paths = fs.read_dir(&PROMPTS_DIR).await?;
        let mut prompts = Vec::new();

        while let Some(path_result) = paths.next().await {
            let path = match path_result {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error reading path: {:?}", e);
                    continue;
                }
            };

            if path.extension() == Some(std::ffi::OsStr::new("md")) {
                match fs.load(&path).await {
                    Ok(content) => match Self::parse_metadata(&content) {
                        Ok((metadata, content_body)) => prompts.push(UserPrompt {
                            metadata,
                            content: content_body,
                        }),
                        Err(e) => eprintln!("{}", e),
                    },
                    Err(e) => eprintln!("Failed to load file {}: {}", path.display(), e),
                }
            }
        }

        Ok(prompts)
    }
}

pub fn log_prompts(fs: Arc<dyn Fs>) {
    futures::executor::block_on(async {
        match UserPrompt::list(fs).await {
            Ok(contexts) => {
                for context in contexts {
                    println!("{:#?}", context);
                }
            }
            Err(e) => eprintln!("Failed to list contexts: {}", e),
        }
    });
}
