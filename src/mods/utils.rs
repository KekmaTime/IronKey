use ratatui::layout::Rect;
use std::fs::{File, OpenOptions};
use std::io::Write;
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

pub fn export_password_history(format: &str, passwords: &[String]) -> std::io::Result<String> {
    let filename = format!("password_history.{}", format);
    let mut file = File::create(&filename)?;
    match format {
        "json" => {
            write!(file, "{}", serde_json::to_string(passwords)?)?;
        }
        "csv" => {
            for password in passwords {
                writeln!(file, "{}", password)?;
            }
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid format",
            ))
        }
    }
    Ok(filename)
}
