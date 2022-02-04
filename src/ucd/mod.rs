#![allow(dead_code)]
mod bidi_class;
mod general_category;

use self::bidi_class::{PARAGRAPH_SEPARATOR, SEGMENT_SEPARATOR, WHITE_SPACE};
use self::general_category::{CONTROL, MARK, MODIFIER_SYMBOL, NUMBER, PUNCTUATION};

pub fn word_separator(c: char) -> bool {
    WHITE_SPACE.contains_char(c) || SEGMENT_SEPARATOR.contains_char(c)
}

pub fn modifier(c: char) -> bool {
    MODIFIER_SYMBOL.contains_char(c)
}

pub fn combining_mark(c: char) -> bool {
    MARK.contains_char(c)
}

pub fn control(c: char) -> bool {
    !paragraph_separator(c) && !SEGMENT_SEPARATOR.contains_char(c) && CONTROL.contains_char(c)
}

pub fn paragraph_separator(c: char) -> bool {
    PARAGRAPH_SEPARATOR.contains_char(c)
}

pub fn punctuation(c: char) -> bool {
    PUNCTUATION.contains_char(c)
}

pub fn number(c: char) -> bool {
    NUMBER.contains_char(c)
}
