use crate::art_char::ArtChar;
use std::sync::LazyLock;
use std::sync::RwLock;

use bson::document::Document;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Serialize, Deserialize, Debug)]
struct Chars {
    content: Vec<ArtChar>,
}

pub static FULL_CHARACTERS: LazyLock<Vec<ArtChar>> = LazyLock::new(|| {
    let mut reader = Cursor::new(include_bytes!("../art_chars/full_chars.bson"));
    let document = Document::from_reader(&mut reader).unwrap();
    let chars: Chars = bson::deserialize_from_document(document).unwrap();
    chars.content
});

pub static COMPACT_CHARACTERS: LazyLock<Vec<ArtChar>> = LazyLock::new(|| {
    let mut reader = Cursor::new(include_bytes!("../art_chars/compact_chars.bson"));
    let document = Document::from_reader(&mut reader).unwrap();
    let chars: Chars = bson::deserialize_from_document(document).unwrap();
    chars.content
});

pub static SHORT_CHARACTERS: LazyLock<Vec<ArtChar>> = LazyLock::new(|| {
    let mut reader = Cursor::new(include_bytes!("../art_chars/short_chars.bson"));
    let document = Document::from_reader(&mut reader).unwrap();
    let chars: Chars = bson::deserialize_from_document(document).unwrap();
    chars.content
});

pub static DESIRED_CHARSET: RwLock<SizeKind> = RwLock::new(SizeKind::Compact);

#[derive(PartialEq, Clone, Copy)]
pub enum SizeKind {
    Short,
    Compact,
    Full,
}

pub fn set_charset(kind: SizeKind) {
    let mut inner = DESIRED_CHARSET.write().unwrap();
    *inner = kind;
}

/*
pub fn set_charset(kind: SizeKind) {
    let mut inner = DESIRED_CHARSET.write().unwrap().deref_mut();
    let take_ownership = *inner;
}




*/
