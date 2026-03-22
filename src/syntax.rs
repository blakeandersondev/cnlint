use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    CStyle,
    HashLine,
    Sql,
    Rust,
    Shell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommentSyntax {
    pub language: Language,
    pub line_markers: &'static [&'static str],
    pub block_marker: Option<(&'static str, &'static str)>,
    pub supports_regex_literals: bool,
    pub supports_backticks: bool,
}

const C_STYLE_SYNTAX: CommentSyntax = CommentSyntax {
    language: Language::CStyle,
    line_markers: &["//"],
    block_marker: Some(("/*", "*/")),
    supports_regex_literals: false,
    supports_backticks: false,
};

const HASH_LINE_SYNTAX: CommentSyntax = CommentSyntax {
    language: Language::HashLine,
    line_markers: &["#"],
    block_marker: None,
    supports_regex_literals: false,
    supports_backticks: false,
};

const SQL_SYNTAX: CommentSyntax = CommentSyntax {
    language: Language::Sql,
    line_markers: &["--"],
    block_marker: Some(("/*", "*/")),
    supports_regex_literals: false,
    supports_backticks: false,
};

const RUST_SYNTAX: CommentSyntax = CommentSyntax {
    language: Language::Rust,
    line_markers: &["//"],
    block_marker: Some(("/*", "*/")),
    supports_regex_literals: false,
    supports_backticks: false,
};

const SHELL_SYNTAX: CommentSyntax = CommentSyntax {
    language: Language::Shell,
    line_markers: &["#"],
    block_marker: None,
    supports_regex_literals: false,
    supports_backticks: true,
};

pub fn comment_syntax_for_path(path: &Path) -> Option<CommentSyntax> {
    let extension = path.extension()?.to_str()?.to_ascii_lowercase();

    match extension.as_str() {
        "java" | "go" | "js" | "jsx" | "ts" | "tsx" | "kt" | "c" | "cc" | "cpp" | "h" | "hpp" => {
            Some(CommentSyntax {
                supports_regex_literals: matches!(extension.as_str(), "js" | "jsx" | "ts" | "tsx"),
                ..C_STYLE_SYNTAX
            })
        }
        "rs" => Some(RUST_SYNTAX),
        "py" | "yaml" | "yml" => Some(HASH_LINE_SYNTAX),
        "sh" | "bash" | "zsh" => Some(SHELL_SYNTAX),
        "sql" => Some(SQL_SYNTAX),
        _ => None,
    }
}
