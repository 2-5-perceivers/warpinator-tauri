use std::collections::VecDeque;

use crate::filesystem::vfs::{VirtualFilesystemError, list_dir};

pub struct WalkDirEntry {
    pub is_dir: bool,
    pub path: String,
    pub name: String,
    pub base_path: String,
}

impl WalkDirEntry {
    pub fn relative_path(&self) -> String {
        if self.base_path.is_empty() {
            self.name.clone()
        } else {
            format!("{}/{}", self.base_path, self.name)
        }
    }

    #[inline]
    pub fn is_file(&self) -> bool {
        !self.is_dir
    }

    #[inline]
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
}

pub struct VirtualWalkDir {
    queue: VecDeque<WalkDirEntry>,
}

impl VirtualWalkDir {
    pub fn new(root_path: String, root_name: String) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(WalkDirEntry {
            is_dir: true,
            path: root_path,
            name: root_name,
            base_path: "".to_string(),
        });
        VirtualWalkDir { queue }
    }

    pub async fn next(&mut self) -> Result<Option<WalkDirEntry>, VirtualFilesystemError> {
        while let Some(current_entry) = self.queue.pop_front() {
            if current_entry.is_dir {
                match list_dir(&current_entry.path).await {
                    Ok(children) => {
                        for child in children {
                            self.queue.push_back(WalkDirEntry {
                                is_dir: child.is_dir,
                                name: child.name,
                                path: child.path,
                                base_path: current_entry.relative_path(),
                            });
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to list directory: {}", e);
                    }
                }
            }

            return Ok(Some(current_entry));
        }

        Ok(None)
    }
}
