use crate::dictionary::Dictionary;
use crate::styled_text::{Style, StyledText};
use crossterm::style::Color;

pub fn style_example(dictionary: &Dictionary, example: &str, target: &str) -> Vec<StyledText> {
    let color = Color::DarkGrey;
    example
        .chars()
        .fold(Vec::<String>::new(), |mut acc, c| {
            if c.is_alphabetic() {
                if let Some(last) = acc.last_mut() {
                    if last.chars().all(|c| c.is_alphabetic() || c == '-') {
                        *last = format!("{}{}", last, c);
                        return acc;
                    }
                }
                acc.push(c.to_string());
            } else {
                acc.push(c.to_string());
            }
            acc
        })
        .into_iter()
        .map(|word| {
            if word.chars().all(|c| c.is_alphabetic() || c == '-') {
                let word_in_sentence = dictionary.get_base_form(word.as_str());
                let target_word = dictionary.get_base_form(target);
                if word_in_sentence == target_word {
                    StyledText::new(word.as_str(), color, Style::BoldUnderline)
                } else {
                    StyledText::new(word.as_str(), color, Style::Plain)
                }
            } else {
                StyledText::new(word.as_str(), color, Style::Plain)
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::styled_text::Style;
    use crossterm::style::Color;

    #[test]
    fn test_style_example_normal() {
        let dictionary = Dictionary::new();
        let example = "be kind.";
        let color = Color::DarkGrey;
        let styled = style_example(&dictionary, example, "be");

        assert_eq!(
            styled[0],
            StyledText::new("be", color, Style::BoldUnderline)
        );
        assert_eq!(styled[1], StyledText::new(" ", color, Style::Plain));
        assert_eq!(styled[2], StyledText::new("kind", color, Style::Plain));
    }

    #[test]
    fn test_style_example_empty() {
        let dictionary = Dictionary::new();
        let example = "";
        let styled = style_example(&dictionary, example, "am");

        assert_eq!(styled.len(), 0);
    }

    #[test]
    fn test_style_example_conjugation() {
        let dictionary = Dictionary::new();
        let example = "I ate a student.";
        let color = Color::DarkGrey;
        let styled = style_example(&dictionary, example, "eat");

        assert_eq!(styled[0], StyledText::new("I", color, Style::Plain));
        assert_eq!(styled[1], StyledText::new(" ", color, Style::Plain));
        assert_eq!(
            styled[2],
            StyledText::new("ate", color, Style::BoldUnderline)
        );
        assert_eq!(styled[3], StyledText::new(" ", color, Style::Plain));
        assert_eq!(styled[4], StyledText::new("a", color, Style::Plain));
        assert_eq!(styled[5], StyledText::new(" ", color, Style::Plain));
        assert_eq!(styled[6], StyledText::new("student", color, Style::Plain));
    }

    #[test]
    fn test_style_example_located_end_with_dot() {
        let dictionary = Dictionary::new();
        let example = "I am.";
        let color = Color::DarkGrey;
        let styled = style_example(&dictionary, example, "be");

        assert_eq!(styled[0], StyledText::new("I", color, Style::Plain));
        assert_eq!(styled[1], StyledText::new(" ", color, Style::Plain));
        assert_eq!(
            styled[2],
            StyledText::new("am", color, Style::BoldUnderline)
        );
        assert_eq!(styled[3], StyledText::new(".", color, Style::Plain));
    }

    #[test]
    fn test_style_example_located_end_with_comma() {
        let dictionary = Dictionary::new();
        let example = "I am,";
        let color = Color::DarkGrey;
        let styled = style_example(&dictionary, example, "be");

        assert_eq!(styled[0], StyledText::new("I", color, Style::Plain));
        assert_eq!(styled[1], StyledText::new(" ", color, Style::Plain));
        assert_eq!(
            styled[2],
            StyledText::new("am", color, Style::BoldUnderline)
        );
        assert_eq!(styled[3], StyledText::new(",", color, Style::Plain));
    }

    #[test]
    fn test_style_example_located_end_with_exclamation() {
        let dictionary = Dictionary::new();
        let example = "I am!";
        let color = Color::DarkGrey;
        let styled = style_example(&dictionary, example, "be");

        assert_eq!(styled[0], StyledText::new("I", color, Style::Plain));
        assert_eq!(styled[1], StyledText::new(" ", color, Style::Plain));
        assert_eq!(
            styled[2],
            StyledText::new("am", color, Style::BoldUnderline)
        );
        assert_eq!(styled[3], StyledText::new("!", color, Style::Plain));
    }

    #[test]
    fn test_style_example_located_end_with_question() {
        let dictionary = Dictionary::new();
        let example = "I am?";
        let color = Color::DarkGrey;
        let styled = style_example(&dictionary, example, "be");

        assert_eq!(styled[0], StyledText::new("I", color, Style::Plain));
        assert_eq!(styled[1], StyledText::new(" ", color, Style::Plain));
        assert_eq!(
            styled[2],
            StyledText::new("am", color, Style::BoldUnderline)
        );
        assert_eq!(styled[3], StyledText::new("?", color, Style::Plain));
    }

    #[test]
    fn test_style_example_located_end_with_semicolon() {
        let dictionary = Dictionary::new();
        let example = "I am;";
        let color = Color::DarkGrey;
        let styled = style_example(&dictionary, example, "be");

        assert_eq!(styled[0], StyledText::new("I", color, Style::Plain));
        assert_eq!(styled[1], StyledText::new(" ", color, Style::Plain));
        assert_eq!(
            styled[2],
            StyledText::new("am", color, Style::BoldUnderline)
        );
        assert_eq!(styled[3], StyledText::new(";", color, Style::Plain));
    }

    #[test]
    fn test_style_example_located_end_with_colon() {
        let dictionary = Dictionary::new();
        let example = "I am:";
        let color = Color::DarkGrey;
        let styled = style_example(&dictionary, example, "be");

        assert_eq!(styled[0], StyledText::new("I", color, Style::Plain));
        assert_eq!(styled[1], StyledText::new(" ", color, Style::Plain));
        assert_eq!(
            styled[2],
            StyledText::new("am", color, Style::BoldUnderline)
        );
        assert_eq!(styled[3], StyledText::new(":", color, Style::Plain));
    }

    #[test]
    fn test_style_example_located_end_with_semicolon_and_space() {
        let dictionary = Dictionary::new();
        let example = "I am; ";
        let color = Color::DarkGrey;
        let styled = style_example(&dictionary, example, "be");

        assert_eq!(styled[0], StyledText::new("I", color, Style::Plain));
        assert_eq!(styled[1], StyledText::new(" ", color, Style::Plain));
        assert_eq!(
            styled[2],
            StyledText::new("am", color, Style::BoldUnderline)
        );
        assert_eq!(styled[3], StyledText::new(";", color, Style::Plain));
    }

    #[test]
    fn test_style_example_between_non_alphabets() {
        let dictionary = Dictionary::new();
        let example = "I :!\"am\"?:";
        let color = Color::DarkGrey;
        let styled = style_example(&dictionary, example, "be");

        assert_eq!(styled[0], StyledText::new("I", color, Style::Plain));
        assert_eq!(styled[1], StyledText::new(" ", color, Style::Plain));
        assert_eq!(styled[2], StyledText::new(":", color, Style::Plain));
        assert_eq!(styled[3], StyledText::new("!", color, Style::Plain));
        assert_eq!(styled[4], StyledText::new("\"", color, Style::Plain));
        assert_eq!(
            styled[5],
            StyledText::new("am", color, Style::BoldUnderline)
        );
        assert_eq!(styled[6], StyledText::new("\"", color, Style::Plain));
        assert_eq!(styled[7], StyledText::new("?", color, Style::Plain));
        assert_eq!(styled[8], StyledText::new(":", color, Style::Plain));
    }
}
