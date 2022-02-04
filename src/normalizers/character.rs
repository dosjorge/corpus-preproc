use clap::Parser;
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

use crate::ucd::{combining_mark, control, modifier, paragraph_separator, word_separator};

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharNormalization {
    #[clap(
        name = "char.enabled",
        short = 'n',
        help = "Perform NFKC and whitespace normalization"
    )]
    pub enabled: bool,
    #[clap(
        short = 'm',
        help = "Keep modifiers and marks on normalization",
        requires = "char.enabled"
    )]
    keep_modifiers_and_marks: bool,
    #[clap(short = 'l', help = "Perform case-folding", requires = "char.enabled")]
    lowercase: bool,
}

impl Default for CharNormalization {
    fn default() -> CharNormalization {
        CharNormalization {
            enabled: true,
            keep_modifiers_and_marks: false,
            lowercase: true,
        }
    }
}

pub const LINE_SEPARATOR: char = '\n';
pub const WORD_SEPARATOR: char = ' ';

fn char_filter(keep_modifiers_and_marks: bool) -> impl Fn(char) -> Option<char> {
    move |c| {
        if word_separator(c) {
            return Some(WORD_SEPARATOR);
        }
        if paragraph_separator(c) {
            return Some(LINE_SEPARATOR);
        }
        if control(c) {
            return None;
        }
        if !keep_modifiers_and_marks && (modifier(c) || combining_mark(c)) {
            return None;
        }
        Some(c)
    }
}

pub fn clean_control(text: String) -> String {
    text.chars().filter(|c| !control(*c)).collect::<String>()
}

pub fn run(text: String, opts: CharNormalization) -> String {
    let text = text
        .nfkd()
        .filter_map(char_filter(opts.keep_modifiers_and_marks))
        .nfkc()
        .collect::<String>();

    if opts.lowercase {
        text.to_lowercase()
    } else {
        text
    }
}
