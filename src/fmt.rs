use crate::art_str::ArtString;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum Align {
    #[default]
    Right,
    Left,
    Center,
}

pub struct Formatter<'a> {
    pub(crate) content: &'a str,
    pub(crate) line_limit: usize,
    pub(crate) align: Align,
    pub(crate) padding_spacer: char,
    pub(crate) word_spacer: &'a str,
    pub(crate) letter_spacer: char,
    pub(crate) modifiers: Vec<Box<dyn FnOnce(String) -> String>>,
}

impl<'a> Default for Formatter<'a> {
    fn default() -> Self {
        let uppercase = |string: String| string.to_uppercase();
        Self {
            content: "",
            line_limit: 26,
            align: Align::default(),
            padding_spacer: ' ',
            word_spacer: "  ",
            letter_spacer: '-',
            modifiers: vec![Box::new(uppercase)],
        }
    }
}

impl<'a> std::convert::From<&'a str> for Formatter<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            content: value,
            ..Self::default()
        }
    }
}

impl<'a> Formatter<'a> {
    pub fn with_line_limit(mut self, value: usize) -> Self {
        self.line_limit = value;
        self
    }

    pub fn with_align(mut self, value: Align) -> Self {
        self.align = value;
        self
    }

    pub fn with_padding_spacer(mut self, value: char) -> Self {
        self.padding_spacer = value;
        self
    }

    pub fn with_word_spacer(mut self, value: &'a str) -> Self {
        self.word_spacer = value;
        self
    }

    pub fn with_letter_spacer(mut self, value: char) -> Self {
        self.letter_spacer = value;
        self
    }

    // Modificador de pasar todo a mayusculas
    // Modificador de espaciado (cambiar " " por "  ")
    pub fn add_modifier(mut self, value: Box<dyn FnOnce(String) -> String>) -> Self {
        self.modifiers.push(value);
        self
    }

    pub fn fmt(self) -> String {
        //Aplicar modificadores
        let modified = self
            .modifiers
            .into_iter()
            .fold(self.content.to_string(), |acc, f| f(acc));

        let spaced_out_letters = Formatter::space_out_letters(modified, self.letter_spacer);
        let adjusted = Formatter::adjust(spaced_out_letters, self.line_limit, self.word_spacer);
        let aligned = Formatter::align(adjusted, self.align, self.line_limit, self.padding_spacer);
        aligned
    }

    fn space_out_letters(string: String, letter_spacer: char) -> String {
        let mut result = String::with_capacity(string.len() * 2);
        let mut in_word = false;
        let mut first_in_word = true;

        for c in string.chars() {
            if c.is_whitespace() {
                in_word = false;
                result.push(c);
            } else {
                if !in_word {
                    in_word = true;
                    first_in_word = true;
                }
                if !first_in_word {
                    result.push(letter_spacer);
                } else {
                    first_in_word = false;
                }
                result.push(c);
            }
        }
        result
    }

    fn adjust(string: String, line_limit: usize, word_spacer: &str) -> String {
        let mut lines = Vec::<String>::new();
        let mut line = String::with_capacity(line_limit);

        for word in string.split_whitespace() {
            let mut spacer = if line.is_empty() { "" } else { word_spacer };

            let word_len = ArtString::len_prev(word);
            let line_len = ArtString::len_prev(&line);
            let spacer_len = ArtString::len_prev(spacer);

            if word_len > line_limit {
                continue;
            }

            if (line_len + spacer_len + word_len) > line_limit {
                lines.push(std::mem::take(&mut line));
                spacer = "";
            }

            line.push_str(spacer);
            line.push_str(word);
        }
        if !line.is_empty() {
            lines.push(line);
        }
        lines.join("\n")
    }

    fn align(string: String, align: Align, line_limit: usize, padding_spacer: char) -> String {
        let mut new_lines = Vec::new();

        for line in string.lines() {
            let line_len = ArtString::len_prev(&line);

            if line_len > line_limit {
                return string;
            };

            let total_padding = line_limit - line_len;

            let (left_padding, right_padding) = match align {
                Align::Left => (0, total_padding),
                Align::Center => {
                    if total_padding % 2 == 0 {
                        (total_padding / 2, total_padding / 2)
                    } else {
                        ((total_padding / 2) + 1, total_padding / 2)
                    }
                }
                Align::Right => (total_padding, 0),
            };

            let mut aligned_str = String::with_capacity(line_limit);

            (0..left_padding).for_each(|_| aligned_str.push(padding_spacer));
            aligned_str.push_str(&line);
            (0..right_padding).for_each(|_| aligned_str.push(padding_spacer));

            new_lines.push(aligned_str);
        }

        new_lines.join("\n")
    }
}
