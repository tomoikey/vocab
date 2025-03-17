mod dictionary;
mod speaker;
mod styled_text;
mod stylist;

use crate::dictionary::Dictionary;
use crate::speaker::Speaker;
use crate::styled_text::{Style, StyledText, print_styled_text, print_styled_texts};
use crate::stylist::style_example;
use clap::Parser;
use crossterm::event::KeyEventKind;
use crossterm::style::Color;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use env_logger::Builder;
use log::{LevelFilter, error, info};
use rand::rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, Stdout, Write};
use std::process::exit;
use unicode_width::UnicodeWidthStr;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// 単語ファイルのパス (JSON形式)
    #[arg(short, long, default_value = "words.json")]
    file: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Word {
    /// 英単語
    english: String,
    /// 例文
    example: String,
    /// 英単語の日本語訳
    japanese: String,
    /// スキップするかどうか
    #[serde(default)]
    skip: bool,
}

enum Action {
    /// 次へ進む
    Next,
    /// 単語帳全体を完全に暗記するまで単語を出題しないようにする
    MarkMemorized,
    /// 終了
    Quit,
}

fn wait_for_action() -> io::Result<Action> {
    loop {
        if let Event::Key(key_event) = event::read()? {
            match (key_event.code, key_event.kind) {
                (KeyCode::Enter, KeyEventKind::Press) => return Ok(Action::Next),
                (KeyCode::Char('m'), KeyEventKind::Press) => return Ok(Action::MarkMemorized),
                (KeyCode::Char('q'), KeyEventKind::Press) => return Ok(Action::Quit),
                _ => {} // 他のキーは無視する
            }
        }
    }
}

fn prompt_reset(file_path: &str, words: &mut Vec<Word>) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::Clear(ClearType::All))?;

    let prompt =
        "All words have been memorized 🎉 Press 'r' to reset the word list or 'q' to quit.";
    let (cols, rows) = terminal::size()?;
    let prompt_width = UnicodeWidthStr::width(prompt) as u16;
    let prompt_x = cols.saturating_sub(prompt_width) / 2;
    let prompt_y = rows / 2;
    execute!(stdout, cursor::MoveTo(prompt_x, prompt_y))?;
    print!("{}", prompt);
    stdout.flush()?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('r') => {
                    for word in words.iter_mut() {
                        word.skip = false;
                    }
                    let file = File::create(file_path)?;
                    serde_json::to_writer_pretty(file, &words)
                        .expect("JSON への書き出しに失敗しました");
                    break;
                }
                KeyCode::Char('q') => {
                    terminal::disable_raw_mode()?;
                    execute!(stdout, LeaveAlternateScreen)?;
                    exit(0);
                }
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}

fn print_progress(stdout: &mut Stdout, terminal_columns: u16, i: usize, total: usize) {
    let progress = format!("{} / {}", i + 1, total);
    let prog_width = UnicodeWidthStr::width(progress.as_str()) as u16;
    let prog_x = terminal_columns.saturating_sub(prog_width) / 2;
    execute!(stdout, cursor::MoveTo(prog_x, 1)).unwrap();

    let progress = StyledText::new(progress.as_str(), Color::DarkCyan, Style::Underline);
    print_styled_text(stdout, &progress);
}

fn print_question(
    stdout: &mut Stdout,
    dictionary: &Dictionary,
    cols: u16,
    rows: u16,
    english: &str,
    example: &str,
) -> (u16, u16) {
    let main_width = UnicodeWidthStr::width(english) as u16;
    let x = cols.saturating_sub(main_width) / 2;
    let y = rows / 2 - 2;
    execute!(stdout, cursor::MoveTo(x, y)).unwrap();

    print_styled_text(
        stdout,
        &StyledText::new(english, Color::Yellow, Style::Bold),
    );

    let x = cols.saturating_sub(UnicodeWidthStr::width(example) as u16) / 2;
    let y = y + 1;
    execute!(stdout, cursor::MoveTo(x, y)).unwrap();
    let words = style_example(dictionary, format!("({example})").as_str(), english);
    print_styled_texts(stdout, &words);

    (x, y)
}

fn print_instructions(
    stdout: &mut Stdout,
    terminal_columns: u16,
    terminal_rows: u16,
) -> io::Result<()> {
    let instructions = "(q)uit, (m)ark memorized, (ret) next";
    let instr_width = UnicodeWidthStr::width(instructions) as u16;
    let x = terminal_columns.saturating_sub(instr_width) - 1;
    let y = terminal_rows.saturating_sub(1);
    execute!(stdout, cursor::MoveTo(x, y))?;
    print!("{}", instructions);
    stdout.flush()?;
    Ok(())
}

fn clear_screen(stdout: &mut Stdout) -> io::Result<()> {
    execute!(stdout, terminal::Clear(ClearType::All))?;
    Ok(())
}

fn switch_to_alternate_screen(stdout: &mut Stdout) -> io::Result<()> {
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    execute!(stdout, cursor::Hide)?;
    Ok(())
}

fn switch_to_main_screen(stdout: &mut Stdout) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::Show, LeaveAlternateScreen)?;
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    println!();
    stdout.flush()?;
    Ok(())
}

fn read_words(file_path: &str) -> io::Result<(Vec<Word>, Vec<usize>)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let words: Vec<Word> = serde_json::from_reader(reader).expect("JSON のパースに失敗しました");

    let indices: Vec<usize> = words
        .iter()
        .enumerate()
        .filter(|(_, word)| !word.skip)
        .map(|(i, _)| i)
        .collect();

    Ok((words, indices))
}

fn run() -> io::Result<()> {
    let Args { file } = Args::parse();
    info!("Initializing tokenizer...");
    let dictionary = Dictionary::new();
    info!("Loaded tokenizer successfully");

    info!("Loading words...");
    let (mut words, mut indices) = read_words(file.as_str())?;
    info!("Loaded words successfully");

    let mut speaker = Speaker::new();

    // 出題対象が存在しない場合、リセット操作を促す
    if indices.is_empty() {
        prompt_reset(file.as_str(), &mut words)?;
        indices = words
            .iter()
            .enumerate()
            .filter(|(_, word)| !word.skip)
            .map(|(i, _)| i)
            .collect();
    }

    let mut rng = rng();
    indices.shuffle(&mut rng);

    let mut stdout = io::stdout();

    switch_to_alternate_screen(&mut stdout)?;

    for (i, &idx) in indices.iter().enumerate() {
        let word = &words[idx];
        speaker.speak(word.english.as_str());

        clear_screen(&mut stdout)?;

        let (cols, rows) = terminal::size()?;

        print_instructions(&mut stdout, cols, rows)?;

        print_progress(&mut stdout, cols, i, indices.len());
        let (_, y) = print_question(
            &mut stdout,
            &dictionary,
            cols,
            rows,
            word.english.as_str(),
            word.example.as_str(),
        );

        match wait_for_action()? {
            Action::Next => {
                let jap_width = UnicodeWidthStr::width(word.japanese.as_str()) as u16;
                let jap_x = cols.saturating_sub(jap_width) / 2;
                let jap_y = y + 2;
                execute!(stdout, cursor::MoveTo(jap_x, jap_y))?;
                println!("{}", word.japanese);
                stdout.flush()?;
                match wait_for_action()? {
                    Action::MarkMemorized => {
                        words[idx].skip = true;
                    }
                    Action::Quit => {
                        break;
                    }
                    _ => {}
                }
            }
            Action::MarkMemorized => {
                words[idx].skip = true;
            }
            Action::Quit => {
                break;
            }
        }
    }

    switch_to_main_screen(&mut stdout)?;

    let file = File::create(file.as_str())?;
    serde_json::to_writer_pretty(file, &words).expect("JSON への書き出しに失敗しました");

    Ok(())
}

fn main() -> io::Result<()> {
    Builder::new().filter_level(LevelFilter::Info).init();
    if let Err(e) = run() {
        error!("Error: {}", e);
        exit(1);
    }
    Ok(())
}
