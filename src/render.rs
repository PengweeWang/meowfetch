//! Side-by-side logo + info rendering with ANSI-aware width calculation.

const GAP: usize = 3;

fn char_width(c: char) -> usize {
    if c.is_control() {
        return 0;
    }
    match c as u32 {
        0x1100..=0x115F
        | 0x2E80..=0x303E
        | 0x3041..=0x33FF
        | 0x3400..=0x4DBF
        | 0x4E00..=0x9FFF
        | 0xA000..=0xA4CF
        | 0xAC00..=0xD7A3
        | 0xF900..=0xFAFF
        | 0xFE30..=0xFE4F
        | 0xFF00..=0xFF60
        | 0xFFE0..=0xFFE6
        | 0x20000..=0x3FFFD => 2,
        _ => 1,
    }
}

fn display_width(line: &str) -> usize {
    let mut width = 0;
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            match chars.next() {
                Some('[') => {
                    for next in chars.by_ref() {
                        if ('\x40'..='\x7e').contains(&next) {
                            break;
                        }
                    }
                }
                Some(_) => {}
                None => break,
            }
            continue;
        }
        width += char_width(c);
    }
    width
}

pub(crate) fn render(logo: &str, rows: &[String]) {
    let logo_lines: Vec<&str> = logo.lines().collect();
    let logo_width = logo_lines
        .iter()
        .map(|line| display_width(line))
        .max()
        .unwrap_or(0);
    let total = logo_lines.len().max(rows.len());
    let top = if logo_lines.len() < total {
        (total - logo_lines.len()) / 2
    } else {
        0
    };

    for i in 0..total {
        let logo_line = if i >= top && i - top < logo_lines.len() {
            logo_lines[i - top]
        } else {
            ""
        };
        let row = rows.get(i).map(String::as_str).unwrap_or("");
        let pad = logo_width.saturating_sub(display_width(logo_line)) + GAP;
        print!("{logo_line}");
        print!("{:pad$}", "", pad = pad);
        println!("{row}");
    }
}
