//! Logo loading: explicit `--logo` file > `~/.config/.meowrc` > built-in art.

use std::path::PathBuf;

use crate::cli::Args;

fn config_path() -> Option<PathBuf> {
    if let Some(dir) = dirs::config_dir() {
        let path = dir.join(".meowrc");
        if path.exists() {
            return Some(path);
        }
    }
    dirs::home_dir()
        .map(|home| home.join(".config").join(".meowrc"))
        .filter(|path| path.exists())
}

fn unescape(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('\'') => out.push('\''),
            Some('"') => out.push('"'),
            Some('\\') => out.push('\\'),
            Some('x') => {
                let hex: String = chars.by_ref().take(2).collect();
                if hex.eq_ignore_ascii_case("1b") {
                    out.push('\x1b');
                } else {
                    out.push('\\');
                    out.push('x');
                    out.push_str(&hex);
                }
            }
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

fn builtin(cat_type: u8) -> &'static str {
    match cat_type {
        1 => crate::art::CAT_ART_1,
        2 => crate::art::CAT_ART_2,
        _ => crate::art::CAT_ART_3,
    }
}

pub(crate) fn load(args: &Args) -> String {
    let custom = args.logo_file.clone().or_else(config_path);
    if let Some(path) = custom {
        if let Ok(content) = std::fs::read_to_string(&path) {
            if !content.trim().is_empty() {
                return unescape(&content);
            }
        }
    }
    builtin(args.cat_type).to_string()
}
