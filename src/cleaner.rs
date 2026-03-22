use crate::comment::{CommentKind, LintResult, extract_chinese_comments, to_lint_result};
use crate::syntax::CommentSyntax;
use crate::utils::{collect_source_files, finish_with_collected_errors, format_io_error};
use anyhow::Result;
use std::fs;

pub fn clean_path(
    path: &str,
    line: bool,
    block: bool,
    trim_empty: bool,
    dry_run: bool,
    json: bool,
) -> Result<()> {
    let clean_line = line || !block;
    let clean_block = block || !line;

    let mut results: Vec<LintResult> = Vec::new();
    let (files, mut errors) = collect_source_files(path);

    for file in files {
        let file_path = &file.path;
        let syntax = file.syntax;
        let content = file.content;

        if dry_run {
            let matches = extract_chinese_comments(&content, syntax);

            for m in matches {
                let should_include = match m.kind {
                    CommentKind::Line => clean_line,
                    CommentKind::Block => clean_block,
                };

                if !should_include {
                    continue;
                }

                let item = to_lint_result(file_path, &m);

                if json {
                    results.push(item);
                } else if item.start_line == item.end_line {
                    println!("{}:{} {}", item.file, item.start_line, item.kind);
                } else {
                    println!(
                        "{}:{}-{} {}",
                        item.file, item.start_line, item.end_line, item.kind
                    );
                }
            }

            continue;
        }

        let cleaned = clean_content(&content, syntax, clean_line, clean_block, trim_empty);

        if cleaned != content {
            if let Err(error) = fs::write(file_path, cleaned) {
                errors.push(format_io_error(file_path, "write", &error));
                continue;
            }
            println!("cleaned {}", file_path.display());
        }
    }

    if dry_run && json {
        println!("{}", serde_json::to_string_pretty(&results)?);
    }

    finish_with_collected_errors("clean", errors)
}

fn clean_content(
    content: &str,
    syntax: CommentSyntax,
    clean_line: bool,
    clean_block: bool,
    trim_empty: bool,
) -> String {
    let mut ranges: Vec<(usize, usize)> = extract_chinese_comments(content, syntax)
        .into_iter()
        .filter(|comment| match comment.kind {
            CommentKind::Line => clean_line,
            CommentKind::Block => clean_block,
        })
        .map(|comment| (comment.start_byte, comment.end_byte))
        .collect();

    ranges.sort_unstable_by_key(|(start, _)| *start);

    let mut output = String::with_capacity(content.len());
    let mut cursor = 0usize;

    for (start, end) in ranges {
        if start < cursor {
            continue;
        }

        output.push_str(&content[cursor..start]);
        cursor = end;
    }

    output.push_str(&content[cursor..]);

    if trim_empty {
        output = normalize_empty_lines(&output);
    }

    if content.ends_with('\n') && !output.ends_with('\n') {
        output.push('\n');
    }
    output
}

fn normalize_empty_lines(content: &str) -> String {
    let mut result: Vec<String> = Vec::new();
    let mut prev_empty = false;
    let mut started = false;

    for line in content.lines() {
        let is_empty = line.trim().is_empty();

        if is_empty {
            if started && !prev_empty {
                result.push(String::new());
            }
        } else {
            result.push(line.to_string());
            started = true;
        }

        prev_empty = is_empty;
    }

    while result.last().map(|s| s.is_empty()).unwrap_or(false) {
        result.pop();
    }

    result.join("\n")
}
