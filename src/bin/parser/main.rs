use artify::art_char::ArtChar;

fn main() {
    use bson::document::Document;
    use serde::{Deserialize, Serialize};
    use std::io::Cursor;

    #[derive(Serialize, Deserialize, Debug)]
    struct Chars {
        content: Vec<ArtChar>,
    }

    let mut reader = Cursor::new(include_bytes!("../../../art_chars/full_chars.bson"));
    let document = Document::from_reader(&mut reader).unwrap();
    let chars: Chars = bson::deserialize_from_document(document).unwrap();
    println!("{:#?}", chars);
}
