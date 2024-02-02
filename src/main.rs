mod mods;
use mods::utils::*;
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{self, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use rand::distributions::Alphanumeric;
use rand::Rng;
use ratatui::backend::CrosstermBackend;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;

    let mut pass_len: usize = 0;
    let mut input = String::new();

    // First screen
    loop {
        term.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Enter Password Length")
                .style(Style::default().fg(Color::Green).bg(Color::Black))
                .borders(Borders::ALL);
            let area = centered_rect(50, 30, size);
            f.render_widget(block, area);

            let input_block = Paragraph::new(input.as_str())
                .style(Style::default().fg(Color::Green).bg(Color::Black))
                .block(Block::default().borders(Borders::NONE));
            let input_area = centered_rect(45, 20, size);
            f.render_widget(input_block, input_area);
        })?;

        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Char(c) if c.is_digit(10) => {
                    input.push(c);
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                KeyCode::Enter => {
                    pass_len = input.parse::<usize>().unwrap_or_default();
                    break;
                }
                _ => {}
            }
        }
    }

    term.clear()?;

    // Second screen
    let pass: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(pass_len)
        .map(char::from)
        .collect();

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
        })?;

        if let Event::Key(event) = read()? {
            match event.code {
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