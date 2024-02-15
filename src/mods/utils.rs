use ratatui::layout::Rect;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn savepass(filename: &str, password: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)?;
    writeln!(file, "{}", password)
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let padding_x = r
        .width
        .saturating_sub(r.width.saturating_mul(percent_x) / 100)
        / 2;
    let padding_y = r
        .height
        .saturating_sub(r.height.saturating_mul(percent_y) / 100)
        / 2;
    Rect::new(
        r.x + padding_x,
        r.y + padding_y,
        r.width.saturating_sub(padding_x * 2),
        r.height.saturating_sub(padding_y * 2),
    )
}
