use std::{fmt, time::Duration};

use console::{style, Style};
use similar::{ChangeTag, TextDiff};

struct Line(Option<usize>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:<4}", idx + 1),
        }
    }
}

pub fn diff_text(old: &str, new: &str) -> String {
    let mut diff_text = String::new();

    let diff = TextDiff::configure()
        .algorithm(similar::Algorithm::Patience)
        .timeout(Duration::from_millis(500))
        .diff_lines(old, new);

    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            diff_text.push_str(&format!("{:-^1$}", "-", 80));
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, s) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };

                diff_text.push_str(&format!(
                    "{}{} |{}",
                    style(Line(change.old_index())).dim(),
                    style(Line(change.new_index())).dim(),
                    s.apply_to(sign).bold(),
                ));

                for (emphasized, value) in change.iter_strings_lossy() {
                    if emphasized {
                        diff_text.push_str(&s.apply_to(value).underlined().to_string());
                    } else {
                        diff_text.push_str(&s.apply_to(value).to_string());
                    }
                }
                if change.missing_newline() {
                    diff_text.push('\n');
                }
            }
        }
    }

    diff_text
}

#[cfg(test)]
mod tests {
    use crate::text;

    #[ignore = "test to see example output"]
    #[test]
    fn similar_example() {
        let old = "Hello World\nThis is the second line.\nThis is the third.\nHAHAHA\nHAHAHA\nHAHAHA\nHAHAHA\nHAHAHAHAHAHA";
        let new = "Hallo Welt\nThis is the second line.\nThis is life.\nMoar and more\nHAHAHA\nHAHAHA\nHAHAHA\nHAHAHA\nHAHAHAHAHAHA";

        let diff = text::diff_text(old, new);
        println!("{diff}");
    }
}
