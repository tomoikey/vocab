use std::io::{Stdout, Write};
use crossterm::execute;
use crossterm::style::{Attribute, Color};

pub fn print_styled_text(stdout: &mut Stdout, styled_text: &StyledText) {
    set_color(stdout, styled_text.color);
    set_bold(stdout, styled_text.bold);
    set_underline(stdout, styled_text.underline);
    print!("{}", styled_text.text);
    reset_color(stdout);

    stdout.flush().unwrap();
}

pub fn print_styled_texts(stdout: &mut Stdout, styled_texts: &[StyledText]) {
    styled_texts.iter().for_each(|styled_text| {
        print_styled_text(stdout, styled_text);
    });
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyledText {
    text: String,
    color: Color,
    bold: bool,
    underline: bool,
}

impl StyledText {
    pub fn new(text: &str, color: Color, style: Style) -> Self {
        Self {
            text: text.to_string(),
            color,
            bold: style == Style::Bold || style == Style::BoldUnderline,
            underline: style == Style::Underline || style == Style::BoldUnderline,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Plain,
    Bold,
    Underline,
    BoldUnderline,
}

fn set_underline(stdout: &mut Stdout, on: bool) {
    if on {
        execute!(
            stdout,
            crossterm::style::SetAttribute(Attribute::Underlined)
        )
        .unwrap();
    } else {
        execute!(
            stdout,
            crossterm::style::SetAttribute(Attribute::NoUnderline)
        )
        .unwrap();
    }
}

fn set_bold(stdout: &mut Stdout, on: bool) {
    if on {
        execute!(stdout, crossterm::style::SetAttribute(Attribute::Bold)).unwrap();
    } else {
        execute!(
            stdout,
            crossterm::style::SetAttribute(Attribute::NormalIntensity)
        )
        .unwrap();
    }
}

fn set_color(stdout: &mut Stdout, color: Color) {
    execute!(stdout, crossterm::style::SetForegroundColor(color)).unwrap();
}

fn reset_color(stdout: &mut Stdout) {
    execute!(stdout, crossterm::style::ResetColor).unwrap();
}
