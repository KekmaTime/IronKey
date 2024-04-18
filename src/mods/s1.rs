use super::utils::{centered_rect, navigate_list};
use crossterm::event::{read, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListState, Paragraph};
use ratatui::Terminal;
use std::error::Error;
use std::io::Stdout;

pub fn s1(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    options: Vec<&str>,
    selected_options: &mut Vec<bool>,
    mut list_state: &mut ListState,
    input: &mut String,
    pass_len: &mut usize,
) -> std::result::Result<(), Box<dyn Error>> {
    loop {
        term.draw(|f| {
            let size = f.size();
            let title_block = Block::default()
                .title("IronKey")
                .title_alignment(Alignment::Center)
                .borders(Borders::NONE)
                .style(Style::default().fg(Color::Green));
            let title_area = centered_rect(60, 50, size);
            f.render_widget(title_block, title_area);

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
                        .title("Select Options")
                        .title_alignment(Alignment::Center),
                )
                .style(Style::default().fg(Color::Green))
                .highlight_style(Style::default().fg(Color::Green))
                .highlight_symbol("❯ ");

            f.render_stateful_widget(list, centered_rect, &mut list_state);
        })?;
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Right => {
                    break;
                }
                KeyCode::Up | KeyCode::Down => {
                    navigate_list(&mut list_state, options.len(), event.code);
                }
                KeyCode::Enter => {
                    if let Some(selected) = list_state.selected() {
                        if selected == 4 {
                            loop {
                                term.draw(|f| {
                                    let size = f.size();
                                    let input_block = Block::default()
                                        .title("Enter Password Length")
                                        .style(Style::default().fg(Color::Green))
                                        .borders(Borders::ALL);
                                    let input_area = centered_rect(60, 20, size);
                                    f.render_widget(input_block, input_area);
                                    let input_paragraph = Paragraph::new(input.as_ref() as &str)
                                        .style(Style::default().fg(Color::Green))
                                        .block(Block::default().borders(Borders::NONE));
                                    let input_paragraph_area = centered_rect(58, 18, size);
                                    f.render_widget(input_paragraph, input_paragraph_area);
                                })?;
                                if let Event::Key(event) = read()? {
                                    match event.code {
                                        KeyCode::Char(c) if c.is_digit(10) => {
                                            input.push(c);
                                        }
                                        KeyCode::Backspace if !input.is_empty() => {
                                            input.pop();
                                        }
                                        KeyCode::Enter => {
                                            *pass_len = input.parse().unwrap_or(0);
                                            break;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        } else {
                            selected_options[selected] = !selected_options[selected];
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
