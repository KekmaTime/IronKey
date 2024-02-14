use rand::distributions::Uniform;
use rand::Rng;
use std::path::PathBuf;
use dirs;
use super::utils::savepass;

pub fn passgen(selected_options: [bool; 4], pass_len: usize) -> std::io::Result<String> {
    let lowercase_letters = "abcdefghijklmnopqrstuvwxyz";
    let uppercase_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "!@#$%^&*()_-+=|[]{};:',.<>?`~";

    let mut charset = String::new();

    if selected_options[0] {
        charset.push_str(lowercase_letters);
    }
    if selected_options[1] {
        charset.push_str(uppercase_letters);
    }
    if selected_options[2] {
        charset.push_str(numbers);
    }
    if selected_options[3] {
        charset.push_str(symbols);
    }

    let charset = charset.chars().collect::<Vec<char>>();
    let dist = Uniform::from(0..charset.len());

    let pass: String = rand::thread_rng()
        .sample_iter(&dist)
        .take(pass_len)
        .map(|index| charset[index])
        .collect();
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let mut file_path = home;
    file_path.push("passwords.txt");
    savepass(file_path.to_str().unwrap(), &pass)?;

    Ok(pass)
}