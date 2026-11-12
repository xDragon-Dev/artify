use crate::art_char::ArtChar;
type Line = Vec<ArtChar>;

#[derive(Clone)]
pub struct ArtString {
    lines: Vec<Line>,
}

// FIXME: Implement as Fmt Trait
impl ArtString {
    pub fn as_readable(&self) -> String {
        let mut readable_fmt = String::new();
        for line in self.lines.iter() {
            for index in 0..=2 {
                line.iter()
                    .for_each(|art_char| readable_fmt.push_str(&art_char.custom_char[index]));
                readable_fmt.push('\n');
            }
        }
        readable_fmt
    }

    pub fn as_usable(&self) -> String {
        let mut readable_fmt = String::new();
        for line in self.lines.iter() {
            for index in 0..=2 {
                line.iter()
                    .for_each(|art_char| readable_fmt.push_str(&art_char.custom_char[index]));
                readable_fmt.push(' ');
            }
        }
        readable_fmt
    }

    pub fn len_prev(string: &str) -> usize {
        string.chars().map(|c| ArtChar::len_prev(&c)).sum()
    }
}

impl std::convert::From<&str> for ArtString {
    fn from(value: &str) -> Self {
        ArtString {
            lines: value
                .split('\n')
                .map(|line| {
                    line.chars()
                        .map(|character| character.into())
                        .collect::<Line>()
                })
                .collect::<Vec<Line>>(),
        }
    }
}

impl std::fmt::Display for ArtString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut readable_fmt = String::new();
        for line in &self.lines {
            for index in 0..=2 {
                line.iter()
                    .for_each(|art_char| readable_fmt.push_str(&art_char.custom_char[index]));
                readable_fmt.push('\n');
            }
        }
        write!(f, "{readable_fmt}")
    }
}
