use clap::Parser;
use serde::{Deserialize, Serialize};
use walkdir::DirEntry;

use crate::cleaners;
use crate::cleaners::html::HtmlCleaning;
use crate::normalizers;
use crate::normalizers::character::CharNormalization;
use crate::normalizers::word::WordNormalization;

#[derive(Parser, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PreprocOpts {
    #[clap(flatten)]
    html_clean: HtmlCleaning,
    #[clap(flatten)]
    char_normalization: CharNormalization,
    #[clap(flatten)]
    word_normalization: WordNormalization,
}

pub fn file_run(opts: PreprocOpts, dir_entry: DirEntry) -> String {
    run(opts, &std::fs::read(dir_entry.path()).unwrap())
}

pub fn run(opts: PreprocOpts, raw_text: &[u8]) -> String {
    let mut text = normalizers::encoding::to_utf8(raw_text);

    if opts.html_clean.enabled {
        text = cleaners::html::run(text, opts.html_clean);
    }

    if opts.char_normalization.enabled {
        text = normalizers::character::run(text, opts.char_normalization);
    }

    if opts.word_normalization.enabled {
        text = normalizers::word::run(text, opts.word_normalization);
    }

    text + "\n"
}
