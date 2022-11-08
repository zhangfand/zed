use fs::{repository::GitRepository, Fs};
use parking_lot::{Mutex, RwLock};
use std::{path::Path, sync::Arc};

pub enum Event {
    UpdatedGitRepositories(Vec<RepositoryEntry>),
}

#[derive(Clone)]
pub struct RepositoryEntry {
    pub(crate) repo: Arc<Mutex<dyn GitRepository>>,

    // Absolute path to the folder containing the .git file or directory
    pub(crate) content_path: Arc<Path>,

    // Absolute path to the actual .git folder.
    // Note: if .git is a file, this points to the folder indicated by the .git file
    pub(crate) git_dir_path: Arc<Path>,
}

impl std::fmt::Debug for RepositoryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RepositoryEntry")
            .field("content_path", &self.content_path)
            .field("git_dir_path", &self.git_dir_path)
            .field("libgit_repository", &"LibGitRepository")
            .finish()
    }
}

impl RepositoryEntry {
    async fn open_canonical_path(
        fs: &dyn Fs,
        canonical_path: Arc<Path>,
    ) -> Option<RepositoryEntry> {
        let repo = fs.open_repo(&canonical_path)?;
        let lock = repo.lock();

        let git_dir_path = fs.canonicalize(lock.path()).await.ok()?.into();
        drop(lock);

        Some(RepositoryEntry {
            repo,
            content_path: canonical_path,
            git_dir_path,
        })
    }

    pub(crate) fn manages_canonical_path(&self, canonical_path: &Path) -> bool {
        canonical_path.starts_with(self.content_path.as_ref())
    }

    pub(crate) fn canonical_path_in_dot_git(&self, canonical_path: &Path) -> bool {
        canonical_path.starts_with(self.git_dir_path.as_ref())
    }
}

pub struct RepositoryStore {
    entries: RwLock<Vec<RepositoryEntry>>,
}

impl RepositoryStore {
    pub fn new() -> Arc<RepositoryStore> {
        Arc::new(RepositoryStore {
            entries: RwLock::new(Vec::new()),
        })
    }

    pub async fn add_canonical_path(&self, fs: &dyn Fs, canonical_path: Arc<Path>) {
        let Err(index) = self
            .entries
            .read()
            .binary_search_by_key(&&canonical_path, |repo| &repo.content_path)
        else {
                return;
        };

        if let Some(entry) = RepositoryEntry::open_canonical_path(fs, canonical_path).await {
            self.entries.write().insert(index, entry);
            todo!("Fire event to project");
        }
    }

    pub fn remove_canonical_path(&self, canonical_path: &Path) {
        let Ok(index) = self
            .entries
            .read()
            .binary_search_by_key(&canonical_path, |repo| &repo.content_path)
        else {
            return;
        };

        self.entries.write().remove(index);
    }

    /// Gives the most specific git repository for a given canonical path
    pub fn repo_for_canonical_path(&self, canonical_path: &Path) -> Option<RepositoryEntry> {
        self.entries
            .read()
            .iter()
            .rev() //entries are ordered lexicographically
            .find(|repo| repo.manages_canonical_path(canonical_path))
            .cloned()
            .or_else(|| todo!("create, clone, add, return"))
    }

    /// Gives the git repository who's .git folder contains the given canonical path
    pub(crate) fn repo_for_canonical_path_in_dot_git(
        &self,
        canonical_path: &Path,
    ) -> Option<RepositoryEntry> {
        // Git repositories cannot be nested, so we don't need to reverse the order
        self.entries
            .read()
            .iter()
            .find(|repo| repo.canonical_path_in_dot_git(canonical_path))
            .cloned()
    }

    /// Gives the git repository who's .git folder contains the given canonical path
    pub(crate) fn canonical_path_in_dot_git(&mut self, canonical_path: &Path) -> bool {
        self.repo_for_canonical_path_in_dot_git(canonical_path)
            .is_some()
    }
}
