use std::path::Path;

use serde::Serialize;

use crate::syntax::{CommentSyntax, Language};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentKind {
    Line,
    Block,
}

#[derive(Debug, Clone)]
pub struct CommentMatch {
    pub kind: CommentKind,
    pub start_line: usize,
    pub end_line: usize,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug, Serialize)]
pub struct LintResult {
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
    pub kind: String,
}

pub fn to_lint_result(path: &Path, comment: &CommentMatch) -> LintResult {
    LintResult {
        file: path.display().to_string(),
        start_line: comment.start_line,
        end_line: comment.end_line,
        kind: format!("{:?}", comment.kind),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScanState {
    Code,
    String {
        quote: u8,
        escape: bool,
    },
    Regex {
        escape: bool,
        in_char_class: bool,
    },
    BlockComment {
        start_byte: usize,
        start_line: usize,
        start_marker: &'static str,
        end_marker: &'static str,
        depth: usize,
        allow_nesting: bool,
    },
    RustRawString {
        hashes: usize,
    },
    SwiftString {
        hashes: usize,
        multiline: bool,
        escape: bool,
    },
}

pub fn contains_chinese(text: &str) -> bool {
    text.chars().any(|c| {
        matches!(
            c as u32,
            0x4E00..=0x9FFF |
            0x3400..=0x4DBF |
            0x20000..=0x2A6DF |
            0x2A700..=0x2B73F |
            0x2B740..=0x2B81F |
            0x2B820..=0x2CEAF |
            0xF900..=0xFAFF
        )
    })
}

pub fn extract_chinese_comments(content: &str, syntax: CommentSyntax) -> Vec<CommentMatch> {
    extract_comments(content, syntax)
        .into_iter()
        .filter(|comment| contains_chinese(&content[comment.start_byte..comment.end_byte]))
        .collect()
}

pub fn extract_comments(content: &str, syntax: CommentSyntax) -> Vec<CommentMatch> {
    let bytes = content.as_bytes();
    let mut comments = Vec::new();
    let mut state = ScanState::Code;
    let mut line = 1usize;
    let mut index = 0usize;

    while index < bytes.len() {
        match state {
            ScanState::Code => {
                if bytes[index] == b'\n' {
                    line += 1;
                    index += 1;
                    continue;
                }

                if let Some(hashes) = detect_rust_raw_string(bytes, index, syntax.language) {
                    state = ScanState::RustRawString { hashes };
                    index += hashes + 2;
                    continue;
                }

                if let Some((hashes, multiline, start_len)) =
                    detect_swift_string_start(bytes, index, syntax.language)
                {
                    state = ScanState::SwiftString {
                        hashes,
                        multiline,
                        escape: false,
                    };
                    index += start_len;
                    continue;
                }

                if let Some((marker, kind)) = detect_comment_start(bytes, index, syntax) {
                    match kind {
                        CommentKind::Line => {
                            let end = line_comment_end(bytes, index);
                            comments.push(CommentMatch {
                                kind,
                                start_line: line,
                                end_line: line,
                                start_byte: index,
                                end_byte: end,
                            });
                            index = end;
                        }
                        CommentKind::Block => {
                            let (start_marker, end_marker) =
                                syntax.block_marker.expect("block marker");
                            state = ScanState::BlockComment {
                                start_byte: index,
                                start_line: line,
                                start_marker,
                                end_marker,
                                depth: 1,
                                allow_nesting: syntax.language == Language::Swift,
                            };
                            index += marker.len();
                        }
                    }
                    continue;
                }

                if is_regex_start(bytes, index, syntax) {
                    state = ScanState::Regex {
                        escape: false,
                        in_char_class: false,
                    };
                    index += 1;
                    continue;
                }

                if is_string_start(bytes[index], syntax) {
                    state = ScanState::String {
                        quote: bytes[index],
                        escape: false,
                    };
                    index += 1;
                    continue;
                }

                index += 1;
            }
            ScanState::String { quote, mut escape } => {
                let current = bytes[index];

                if current == b'\n' {
                    line += 1;
                    if quote != b'`' {
                        state = ScanState::Code;
                    }
                    index += 1;
                    continue;
                }

                if escape {
                    escape = false;
                    state = ScanState::String { quote, escape };
                    index += 1;
                    continue;
                }

                if current == b'\\' {
                    escape = true;
                    state = ScanState::String { quote, escape };
                    index += 1;
                    continue;
                }

                if current == quote {
                    state = ScanState::Code;
                    index += 1;
                    continue;
                }

                index += 1;
            }
            ScanState::Regex {
                mut escape,
                mut in_char_class,
            } => {
                let current = bytes[index];

                if current == b'\n' {
                    line += 1;
                    state = ScanState::Code;
                    index += 1;
                    continue;
                }

                if escape {
                    escape = false;
                    state = ScanState::Regex {
                        escape,
                        in_char_class,
                    };
                    index += 1;
                    continue;
                }

                match current {
                    b'\\' => escape = true,
                    b'[' => in_char_class = true,
                    b']' => in_char_class = false,
                    b'/' if !in_char_class => {
                        state = ScanState::Code;
                        index += 1;
                        continue;
                    }
                    _ => {}
                }

                state = ScanState::Regex {
                    escape,
                    in_char_class,
                };
                index += 1;
            }
            ScanState::BlockComment {
                start_byte,
                start_line,
                start_marker,
                end_marker,
                mut depth,
                allow_nesting,
            } => {
                if bytes[index] == b'\n' {
                    line += 1;
                    index += 1;
                    continue;
                }

                if allow_nesting && starts_with(bytes, index, start_marker.as_bytes()) {
                    depth += 1;
                    state = ScanState::BlockComment {
                        start_byte,
                        start_line,
                        start_marker,
                        end_marker,
                        depth,
                        allow_nesting,
                    };
                    index += start_marker.len();
                    continue;
                }

                if starts_with(bytes, index, end_marker.as_bytes()) {
                    let end = index + end_marker.len();
                    depth -= 1;
                    if depth == 0 {
                        comments.push(CommentMatch {
                            kind: CommentKind::Block,
                            start_line,
                            end_line: line,
                            start_byte,
                            end_byte: end,
                        });
                        state = ScanState::Code;
                    } else {
                        state = ScanState::BlockComment {
                            start_byte,
                            start_line,
                            start_marker,
                            end_marker,
                            depth,
                            allow_nesting,
                        };
                    }
                    index = end;
                    continue;
                }

                index += 1;
            }
            ScanState::RustRawString { hashes } => {
                if bytes[index] == b'\n' {
                    line += 1;
                    index += 1;
                    continue;
                }

                if bytes[index] == b'"' && rust_raw_string_end(bytes, index, hashes) {
                    state = ScanState::Code;
                    index += hashes + 1;
                    continue;
                }

                index += 1;
            }
            ScanState::SwiftString {
                hashes,
                multiline,
                mut escape,
            } => {
                let current = bytes[index];

                if current == b'\n' {
                    line += 1;
                    if !multiline {
                        state = ScanState::Code;
                    }
                    index += 1;
                    continue;
                }

                if multiline {
                    if swift_string_end(bytes, index, hashes, true) {
                        state = ScanState::Code;
                        index += 3 + hashes;
                        continue;
                    }

                    index += 1;
                    continue;
                }

                if hashes == 0 {
                    if escape {
                        escape = false;
                        state = ScanState::SwiftString {
                            hashes,
                            multiline,
                            escape,
                        };
                        index += 1;
                        continue;
                    }

                    if current == b'\\' {
                        escape = true;
                        state = ScanState::SwiftString {
                            hashes,
                            multiline,
                            escape,
                        };
                        index += 1;
                        continue;
                    }
                }

                if swift_string_end(bytes, index, hashes, false) {
                    state = ScanState::Code;
                    index += 1 + hashes;
                    continue;
                }

                state = ScanState::SwiftString {
                    hashes,
                    multiline,
                    escape,
                };
                index += 1;
            }
        }
    }

    comments
}

fn detect_comment_start(
    bytes: &[u8],
    index: usize,
    syntax: CommentSyntax,
) -> Option<(&'static str, CommentKind)> {
    for marker in syntax.line_markers {
        if starts_with(bytes, index, marker.as_bytes()) {
            return Some((marker, CommentKind::Line));
        }
    }

    if let Some((start, _)) = syntax.block_marker
        && starts_with(bytes, index, start.as_bytes())
    {
        return Some((start, CommentKind::Block));
    }

    None
}

fn line_comment_end(bytes: &[u8], start: usize) -> usize {
    let mut index = start;
    while index < bytes.len() && bytes[index] != b'\n' {
        index += 1;
    }
    index
}

fn is_string_start(current: u8, syntax: CommentSyntax) -> bool {
    matches!(current, b'"' | b'\'') || (syntax.supports_backticks && current == b'`')
}

fn is_regex_start(bytes: &[u8], index: usize, syntax: CommentSyntax) -> bool {
    if !syntax.supports_regex_literals || bytes[index] != b'/' {
        return false;
    }

    if index + 1 >= bytes.len() {
        return false;
    }

    let next = bytes[index + 1];
    if next == b'/' || next == b'*' || next.is_ascii_whitespace() {
        return false;
    }

    let prev = previous_significant_byte(bytes, index);
    match prev {
        None => true,
        Some(ch) if b"=([{!?:;,<>+-*%^&|~".contains(&ch) => true,
        Some(_) => previous_keyword(bytes, index)
            .map(|keyword| matches!(keyword, "return" | "case" | "throw" | "in" | "of"))
            .unwrap_or(false),
    }
}

fn previous_significant_byte(bytes: &[u8], index: usize) -> Option<u8> {
    let mut cursor = index;
    while cursor > 0 {
        cursor -= 1;
        let byte = bytes[cursor];
        if !byte.is_ascii_whitespace() {
            return Some(byte);
        }
    }
    None
}

fn previous_keyword(bytes: &[u8], index: usize) -> Option<&str> {
    let mut end = index;
    while end > 0 && bytes[end - 1].is_ascii_whitespace() {
        end -= 1;
    }

    let mut start = end;
    while start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
        start -= 1;
    }

    if start == end {
        return None;
    }

    std::str::from_utf8(&bytes[start..end]).ok()
}

fn detect_rust_raw_string(bytes: &[u8], index: usize, language: Language) -> Option<usize> {
    if language != Language::Rust || bytes[index] != b'r' {
        return None;
    }

    let mut cursor = index + 1;
    let mut hashes = 0usize;

    while cursor < bytes.len() && bytes[cursor] == b'#' {
        hashes += 1;
        cursor += 1;
    }

    if cursor < bytes.len() && bytes[cursor] == b'"' {
        Some(hashes)
    } else {
        None
    }
}

fn rust_raw_string_end(bytes: &[u8], index: usize, hashes: usize) -> bool {
    if index + hashes >= bytes.len() {
        return false;
    }

    for offset in 0..hashes {
        if bytes[index + 1 + offset] != b'#' {
            return false;
        }
    }

    true
}

fn detect_swift_string_start(
    bytes: &[u8],
    index: usize,
    language: Language,
) -> Option<(usize, bool, usize)> {
    if language != Language::Swift {
        return None;
    }

    let mut cursor = index;
    let mut hashes = 0usize;
    while cursor < bytes.len() && bytes[cursor] == b'#' {
        hashes += 1;
        cursor += 1;
    }

    if starts_with(bytes, cursor, b"\"\"\"") {
        return Some((hashes, true, hashes + 3));
    }

    if bytes.get(cursor) == Some(&b'"') {
        return Some((hashes, false, hashes + 1));
    }

    None
}

fn swift_string_end(bytes: &[u8], index: usize, hashes: usize, multiline: bool) -> bool {
    let quote_len = if multiline { 3 } else { 1 };

    if multiline {
        if !starts_with(bytes, index, b"\"\"\"") {
            return false;
        }
    } else if bytes.get(index) != Some(&b'"') {
        return false;
    }

    for offset in 0..hashes {
        if bytes.get(index + quote_len + offset) != Some(&b'#') {
            return false;
        }
    }

    true
}

fn starts_with(bytes: &[u8], index: usize, needle: &[u8]) -> bool {
    bytes
        .get(index..index + needle.len())
        .map(|slice| slice == needle)
        .unwrap_or(false)
}
