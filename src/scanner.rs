use anyhow::Result;

use crate::comment::{LintResult, extract_chinese_comments, to_lint_result};
use crate::utils::{collect_source_files, finish_with_collected_errors};

pub fn scan_path(path: &str, json: bool) -> Result<()> {
    let mut results: Vec<LintResult> = Vec::new();
    let (files, errors) = collect_source_files(path);

    for file in files {
        let matches = extract_chinese_comments(&file.content, file.syntax);
        for m in matches {
            let item = to_lint_result(&file.path, &m);

            if json {
                results.push(item);
            } else if item.start_line == item.end_line {
                println!("{}:{}", item.file, item.start_line);
            } else {
                println!("{}:{}-{}", item.file, item.start_line, item.end_line);
            }
        }
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&results)?);
    }

    finish_with_collected_errors("scan", errors)
}

pub fn check_path(path: &str) -> Result<bool> {
    let mut has_issue = false;
    let (files, errors) = collect_source_files(path);

    for file in files {
        let matches = extract_chinese_comments(&file.content, file.syntax);

        if !matches.is_empty() {
            has_issue = true;
        }
    }

    finish_with_collected_errors("check", errors)?;
    Ok(has_issue)
}
