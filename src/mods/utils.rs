use crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ratatui::widgets::ListState;
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
    let content = match format {
        "json" => generate_json_content(passwords)?,
        "csv" => generate_csv_content(passwords)?,
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid format",
            ))
        }
    };
    let filename = format!("password_history.{}", format);
    let mut file = File::create(&filename)?;
    writeln!(file, "{}", content)?;
    Ok(filename)
}

pub fn generate_json_content(passwords: &[String]) -> std::io::Result<String> {
    Ok(serde_json::to_string(passwords)?)
}

pub fn generate_csv_content(passwords: &[String]) -> std::io::Result<String> {
    let csv_content = passwords.join("\n");
    Ok(csv_content)
}
#[cfg(feature = "wayland_support")]
pub fn set_clipboard_content(content: &str) -> Result<(), String> {
    use wl_clipboard_rs::copy::{MimeType, Options, Source};
    let opts = Options::new();
    opts.copy(Source::Bytes(content.to_string().into_bytes().into()), MimeType::Autodetect)
        .map_err(|_| "Failed to copy content to clipboard".to_string())?;
    Ok(())
}

#[cfg(not(feature = "wayland_support"))]
pub fn set_clipboard_content(content: &str) -> Result<(), String> {
    let clipboard = x11_clipboard::Clipboard::new().map_err(|_| "Failed to access clipboard".to_string())?;
    clipboard.store(clipboard.setter.atoms.clipboard, clipboard.setter.atoms.utf8_string, content)
        .map_err(|_| "Failed to copy content to clipboard".to_string())?;
    Ok(())
}

pub fn navigate_list(list_state: &mut ListState, total_options: usize, key_code: KeyCode) {
    if let Some(selected) = list_state.selected() {
        match key_code {
            KeyCode::Up => {
                if selected > 0 {
                    list_state.select(Some(selected - 1));
                }
            }
            KeyCode::Down => {
                if selected < total_options - 1 {
                    list_state.select(Some(selected + 1));
                }
            }
            _ => {}
        }
    }
}
