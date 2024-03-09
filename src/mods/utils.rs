use ratatui::layout::Rect;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;
use clipboard::{ClipboardContext,ClipboardProvider};

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

    let mut all_passwords = passwords.to_vec();
    let home_dir = dirs_next::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?;
    let passwords_file_path = home_dir.join("passwords.txt");
    if let Ok(lines) = read_lines(passwords_file_path) {
        for line in lines {
            if let Ok(password) = line {
                all_passwords.push(password);
            }
        }
    }

    match format {
        "json" => {
            write!(file, "{}", serde_json::to_string(&all_passwords)?)?;
        }
        "csv" => {
            for password in &all_passwords {
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

pub fn set_clipboard_content(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(content.to_owned())?;
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
