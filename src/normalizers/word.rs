use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use crate::normalizers::character::{LINE_SEPARATOR, WORD_SEPARATOR};

// This is needed in order to avoid recompiling the regular expression on a loop.
// See: https://tinyurl.com/rust-regex-loop
lazy_static! {
    static ref NON_WORD: Regex = Regex::new(r"\pP{2}|@").unwrap();
}

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WordNormalization {
    #[clap(
        name = "word.enabled",
        short = 'p',
        help = "Trim punctuation surrounding words"
    )]
    pub enabled: bool,
    #[clap(
        short = 'r',
        help = "Replace odd words with placeholder",
        requires = "char.enabled"
    )]
    replace_pii: bool,
}

impl Default for WordNormalization {
    fn default() -> WordNormalization {
        WordNormalization {
            enabled: true,
            replace_pii: true,
        }
    }
}

pub const PLACEHOLDER: &str = "<unk>";

pub fn run(text: String, opts: WordNormalization) -> String {
    let mut sentences: Vec<String> = Vec::new();
    for line in text.unicode_sentences().map(ToOwned::to_owned) {
        let mut words: Vec<String> = Vec::new();

        for mut word in line.unicode_words().map(ToOwned::to_owned) {
            word = word.trim_matches(|c| !char::is_alphanumeric(c)).to_owned();
            if !word.is_empty() {
                if opts.replace_pii
                    && (NON_WORD.is_match(&word) || word.find(char::is_alphabetic) == None)
                {
                    word = PLACEHOLDER.to_string();
                } else {
                    word = word;
                }
                words.push(word);
            }
        }
        if words.len() > 0 {
            sentences.push(words.join(&WORD_SEPARATOR.to_string()))
        }
    }

    sentences.join(&LINE_SEPARATOR.to_string())
}
