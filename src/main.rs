use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{self, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use rand::distributions::Alphanumeric;
use rand::Rng;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Text;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;

    let mut pass: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(0)
        .map(char::from)
        .collect();
    let mut pass_len: usize = 0;
    loop {
        term.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let title = "IronKey";
            let width = 140;
            let title = format!("{: ^1$}", title, width);

            let block = Block::default()
                .title(&*title)
                .style(Style::default().fg(Color::Green).bg(Color::Black))
                .borders(Borders::ALL);
            f.render_widget(block, chunks[0]);

            let current_length =
                Paragraph::new(Text::from(format!("Password Length: {}", pass_len)))
                    .style(Style::default().fg(Color::Green).bg(Color::Black))
                    .block(Block::default().borders(Borders::ALL));
            f.render_widget(current_length, chunks[0]);

            let paragraph = Paragraph::new(&*pass)
                .style(Style::default().fg(Color::Green).bg(Color::Black))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(paragraph, chunks[1]);
        })?;

        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Char(c) if c.is_digit(10) => {
                    let digit: usize = c.to_digit(10).unwrap() as usize;
                    pass_len = pass_len + 1;
                }
                _ => {
                    pass = rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(pass_len)
                        .map(char::from)
                        .collect();
                }
            }
        }
    }

    let _ = stdout().execute(LeaveAlternateScreen);
    terminal::disable_raw_mode()?;
    Ok(())
}
