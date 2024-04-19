use super::passgen::passgen;
use super::utils::*;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListState, Paragraph};
use ratatui::Terminal;
use std::error::Error;
use std::io::Stdout;
use std::vec;

pub fn s2(
    selected_options: Vec<bool>,
    pass_len: usize,
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    read: &mut dyn FnMut() -> Result<Event, std::io::Error>,
) -> Result<(), Box<dyn Error>> {
    let selected_options_array: [bool; 4] = [
        selected_options.get(0).cloned().unwrap_or_default(),
        selected_options.get(1).cloned().unwrap_or_default(),
        selected_options.get(2).cloned().unwrap_or_default(),
        selected_options.get(3).cloned().unwrap_or_default(),
    ];

    let pass = passgen(selected_options_array, pass_len)?;

    let mut status_message = String::new();

    let options_2nd_screen = vec!["export-json", "export-csv", "view-passwords"];
    let selected_options_2nd_screen = vec![false; options_2nd_screen.len()];
    let mut list_state_2nd_screen = ListState::default();
    if !options_2nd_screen.is_empty() {
        list_state_2nd_screen.select(Some(0));
    }
    // 2nd screen
    loop {
        term.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Generated Password")
                .title_alignment(Alignment::Center)
                .style(Style::default().fg(Color::Green))
                .borders(Borders::ALL);
            let area = centered_rect(60, 40, size);
            f.render_widget(block, area);

            let paragraph = Paragraph::new(&*pass)
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::NONE));
            let area = centered_rect(50, 20, size);
            f.render_widget(paragraph, area);

            let status_paragraph = Paragraph::new(&*status_message)
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::NONE));
            let area = centered_rect(50, 10, size);
            f.render_widget(status_paragraph, area);

            let items_2nd_screen: Vec<_> = options_2nd_screen
                .iter()
                .enumerate()
                .map(|(i, option)| {
                    let symbol = if selected_options_2nd_screen[i] {
                        "[*]"
                    } else {
                        "[ ]"
                    };
                    format!("{} {}", symbol, option)
                })
                .collect();
            let list_2nd_screen = List::new(items_2nd_screen)
                .block(Block::default().borders(Borders::NONE))
                .style(Style::default().fg(Color::Green))
                .highlight_style(Style::default().fg(Color::Green))
                .highlight_symbol("❯ ");
            let rect = Rect::new(size.width - 50, size.height / 2 - 5, 30, 10);
            f.render_stateful_widget(list_2nd_screen, rect, &mut list_state_2nd_screen);
        })?;
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Up | KeyCode::Down => {
                    navigate_list(
                        &mut list_state_2nd_screen,
                        options_2nd_screen.len(),
                        event.code,
                    );
                }
                KeyCode::Enter => {
                    if let Some(selected) = list_state_2nd_screen.selected() {
                        if selected == 0 || selected == 1 {
                            let format = if selected == 0 { "json" } else { "csv" };
                            match export_password_history(format, &[pass.clone()]) {
                                Ok(filename) => loop {
                                    term.draw(|f| {
                                        let size = f.size();
                                        let message = format!(
                                            "Passwords have been exported as {} to {}",
                                            format, filename
                                        );
                                        let message_block = Block::default()
                                            .title("Successful Export!")
                                            .title_alignment(Alignment::Center)
                                            .style(Style::default().fg(Color::Green))
                                            .borders(Borders::ALL);
                                        let message_area = centered_rect(60, 20, size);
                                        f.render_widget(message_block, message_area);
                                        let message_paragraph =
                                            Paragraph::new(message.as_ref() as &str)
                                                .style(Style::default().fg(Color::Green))
                                                .block(Block::default().borders(Borders::NONE));
                                        let message_paragraph_area = centered_rect(58, 18, size);
                                        f.render_widget(message_paragraph, message_paragraph_area);
                                    })?;
                                    if let Event::Key(event) = read()? {
                                        match event.code {
                                            KeyCode::Enter => {
                                                break;
                                            }
                                            KeyCode::Char('q') => {
                                                break;
                                            }
                                            _ => {}
                                        }
                                    }
                                },
                                Err(_e) => {}
                            }
                        }
                    }
                }
                KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                    match set_clipboard_content(&pass) {
                        Err(e) => status_message = e,
                        Ok(_) => status_message = "Password copied to clipboard!".to_string(),
                    }
                }
                KeyCode::Char('q') => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
