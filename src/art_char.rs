use crate::char_set::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct ArtChar {
    character: char,
    pub custom_char: [String; 3],
}

impl ArtChar {
    pub fn len(&self) -> usize {
        self.custom_char[0].chars().count()
    }

    pub fn default() -> Self {
        ArtChar {
            character: ' ',
            custom_char: [String::from("░"), String::from("░"), String::from("░")],
        }
    }

    pub fn len_prev(character: &char) -> usize {
        let charset = match DESIRED_CHARSET.read().unwrap().deref() {
            SizeKind::Short => SHORT_CHARACTERS.deref(),
            SizeKind::Compact => COMPACT_CHARACTERS.deref(),
            SizeKind::Full => FULL_CHARACTERS.deref(),
        };
        for c in charset {
            if c.character.eq(character) {
                return c.len();
            }
        }
        return ArtChar::default().len();
    }
}

impl std::convert::From<char> for ArtChar {
    fn from(value: char) -> Self {
        let charset = match *DESIRED_CHARSET.read().unwrap() {
            SizeKind::Short => SHORT_CHARACTERS.deref(),
            SizeKind::Compact => COMPACT_CHARACTERS.deref(),
            SizeKind::Full => FULL_CHARACTERS.deref(),
        };
        for c in charset {
            if c.character == value {
                return c.clone();
            }
        }
        ArtChar::default()
    }
}
