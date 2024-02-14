mod mods;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{self, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use mods::utils::*;
use mods::passgen::*;
use ratatui::backend::CrosstermBackend;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListState, Paragraph};
use ratatui::Terminal;
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;

    let mut pass_len: usize = 0;
    let mut input = String::new();
    let options = vec!["Uppercase", "Lowercase", "Numbers", "Special Characters"];
    let mut selected_options = vec![false; options.len()];

    let mut list_state = ListState::default();
    if !options.is_empty() {
        list_state.select(Some(0));
    }

    //1st screen
    loop {
        term.draw(|f| {
            let size = f.size();

            let centered_rect = centered_rect(50, 40, size);
            let items: Vec<_> = options
                .iter()
                .enumerate()
                .map(|(i, option)| {
                    let symbol = if selected_options[i] { "[*]" } else { "[ ]" };
                    format!("{} {}", symbol, option)
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Select Options"),
                )
                .style(Style::default().fg(Color::Green).bg(Color::Black))
                .highlight_style(Style::default().fg(Color::Green).bg(Color::Black))
                .highlight_symbol("❯ ");

            f.render_stateful_widget(list, centered_rect, &mut list_state);
        })?;
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Up => {
                    if let Some(selected) = list_state.selected() {
                        if selected > 0 {
                            list_state.select(Some(selected - 1));
                        }
                    }
                }
                KeyCode::Down => {
                    if let Some(selected) = list_state.selected() {
                        if selected < options.len() - 1 {
                            list_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Enter => {
                    if let Some(selected) = list_state.selected() {
                        selected_options[selected] = !selected_options[selected];
                    }
                }
                _ => {}
            }
        }
    }

    term.clear()?;

    // Second screen
    loop {
        term.draw(|f| {
            let size = f.size();

            let centered_rect = centered_rect(50, 40, size);

            let input_block = Paragraph::new(input.as_ref() as &str)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Enter Password Length"),
                )
                .style(Style::default().fg(Color::Green).bg(Color::Black));

            f.render_widget(input_block, centered_rect);
        })?;

        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Char(c) if c.is_digit(10) => {
                    input.push(c);
                }
                KeyCode::Backspace if !input.is_empty() => {
                    input.pop();
                }
                KeyCode::Enter => {
                    pass_len = input.parse().unwrap_or(0);
                    break;
                }
                _ => {}
            }
        }
    }

    term.clear()?;

    let selected_options_array: [bool; 4] = [
    selected_options.get(0).cloned().unwrap_or_default(),
    selected_options.get(1).cloned().unwrap_or_default(),
    selected_options.get(2).cloned().unwrap_or_default(),
    selected_options.get(3).cloned().unwrap_or_default(),
];

    let pass = passgen(selected_options_array, pass_len)?;

    let mut status_message = String::new();

    // Third screen
    loop {
        term.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Generated Password")
                .style(Style::default().fg(Color::Green).bg(Color::Black))
                .borders(Borders::ALL);
            let area = centered_rect(50, 30, size);
            f.render_widget(block, area);

            let paragraph = Paragraph::new(&*pass)
                .style(Style::default().fg(Color::Green).bg(Color::Black))
                .block(Block::default().borders(Borders::NONE));
            let area = centered_rect(40, 20, size);
            f.render_widget(paragraph, area);

            let status_paragraph = Paragraph::new(&*status_message)
                .style(Style::default().fg(Color::Green).bg(Color::Black))
                .block(Block::default().borders(Borders::NONE));
            let area = centered_rect(40, 10, size);
            f.render_widget(status_paragraph, area);
        })?;

        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    ctx.set_contents(pass.clone()).unwrap();
                    status_message = "Password copied to clipboard!".to_string();
                }
                KeyCode::Char('q') => {
                    break;
                }
                _ => {}
            }
        }
    }

    let _ = stdout().execute(LeaveAlternateScreen);
    terminal::disable_raw_mode()?;
    Ok(())
}
