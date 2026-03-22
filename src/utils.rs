use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use walkdir::{DirEntry, Error as WalkDirError, WalkDir};

use crate::syntax::{CommentSyntax, comment_syntax_for_path};

#[derive(Debug)]
pub struct LoadedSourceFile {
    pub path: PathBuf,
    pub syntax: CommentSyntax,
    pub content: String,
}

pub fn should_skip_dir(name: &str) -> bool {
    matches!(
        name,
        ".git" | "node_modules" | "target" | "dist" | "build" | ".idea" | ".vscode"
    )
}

pub fn format_walkdir_error(error: WalkDirError) -> String {
    match error.path() {
        Some(path) => format!("failed to traverse {}: {error}", path.display()),
        None => format!("failed to traverse directory: {error}"),
    }
}

pub fn format_io_error(path: &Path, action: &str, error: &std::io::Error) -> String {
    format!("failed to {action} {}: {error}", path.display())
}

pub fn collect_source_files(path: &str) -> (Vec<LoadedSourceFile>, Vec<String>) {
    let mut files = Vec::new();
    let mut errors = Vec::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|entry| !should_skip_entry(entry))
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                errors.push(format_walkdir_error(error));
                continue;
            }
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.into_path();
        let Some(syntax) = comment_syntax_for_path(&path) else {
            continue;
        };

        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(error) => {
                errors.push(format_io_error(&path, "read", &error));
                continue;
            }
        };

        files.push(LoadedSourceFile {
            path,
            syntax,
            content,
        });
    }

    (files, errors)
}

pub fn finish_with_collected_errors(command: &str, errors: Vec<String>) -> Result<()> {
    if errors.is_empty() {
        return Ok(());
    }

    let preview: Vec<String> = errors.iter().take(10).cloned().collect();
    let mut message = format!(
        "{command} completed with {} file processing error(s):\n{}",
        errors.len(),
        preview.join("\n")
    );

    if errors.len() > preview.len() {
        message.push_str(&format!(
            "\n... and {} more error(s)",
            errors.len() - preview.len()
        ));
    }

    bail!(message)
}

fn should_skip_entry(entry: &DirEntry) -> bool {
    if entry.file_type().is_dir() {
        let name = entry.file_name().to_string_lossy();
        should_skip_dir(&name)
    } else {
        false
    }
}
