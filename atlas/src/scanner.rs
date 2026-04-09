use std::path::Path;
use tokio::fs;

/// Scout result containing file path and its size in lines (mocked/calculated).
#[derive(Debug, Clone)]
pub struct ScoutEntry {
    pub path: String,
    pub lines: usize,
    pub status: String,
}

/// Recursively scan the directory and return a list of scout entries.
pub async fn scan_directory<P: AsRef<Path>>(root: P) -> Vec<ScoutEntry> {
    let mut entries = Vec::new();
    let mut stack = vec![root.as_ref().to_path_buf()];

    while let Some(current_path) = stack.pop() {
        if let Ok(mut dir_entries) = fs::read_dir(&current_path).await {
            while let Ok(Some(entry)) = dir_entries.next_entry().await {
                let path = entry.path();
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                // Basic ignore list
                if file_name == ".git" || file_name == "target" || file_name == "node_modules" {
                    continue;
                }

                if let Ok(metadata) = entry.metadata().await {
                    if metadata.is_dir() {
                        stack.push(path);
                    } else if metadata.is_file() {
                        let rel_path = path
                            .strip_prefix(root.as_ref())
                            .unwrap_or(&path)
                            .to_string_lossy()
                            .into_owned();

                        // Count lines (rudimentary)
                        let lines = if let Ok(content) = fs::read_to_string(&path).await {
                            content.lines().count()
                        } else {
                            0
                        };

                        entries.push(ScoutEntry {
                            path: rel_path,
                            lines,
                            status: "Captured".to_string(),
                        });
                    }
                }
            }
        }
    }

    entries
}
