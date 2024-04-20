# IronKey

IronKey is a Terminal User Interface (TUI) based password generator written in Rust. It leverages the power of Rust's performance and safety features to provide a fast and secure way to generate passwords.

## Features

- **TUI Interface**: IronKey uses a text-based user interface, making it lightweight and easy to use directly from the terminal.

- **Customizable Password Generation**: IronKey generates passwords based on user-selected options. You can choose the length and the types of characters to include in the password.

- **Password Saving**: IronKey saves the generated password to a file in the user's home directory for future reference.

**Exporting Passwords**: After generating a password, you can export your password history to a file in either JSON or CSV format. This option is presented in the second screen after generating a password.

**Clipboard Support**: You can copy the generated password directly to your clipboard by pressing `Ctrl+C` in the second screen, making it easy to use your new password immediately.

## Installation

To install IronKey, you need to have `Rust` & `Git` installed on your machine. If you don't have them installed in your machine, you can install it from the [Rust](https://www.rust-lang.org/tools/install) & [Git](https://git-scm.com/downloads).

Once Rust & Git is installed, you can install IronKey by cloning the repository and building the project:

```sh
git clone https://github.com/KekmaTime/IronKey.git
cd IronKey
cargo build --release
```

or

```sh
cargo install ironkey
```

## Usage

```
ironkey
```

This will start the application & you will be presented with several options for generating a password:

- **Length**: Input the desired length of the password directly.

- **Symbols**: Use the arrow keys to toggle between "Yes" and "No". If "Yes" is selected,  the generated password will include symbol characters.

- **Numbers**: Use the arrow keys to toggle between "Yes" and "No". If "Yes" is selected, the generated password will include numeric characters.

- **Lowercase Characters**: Use the arrow keys to toggle between "Yes" and "No". If "Yes" is selected, the generated password will include lowercase alphabetic characters.

- **Uppercase Characters**: Use the arrow keys to toggle between "Yes" and "No". If "Yes" is selected, the generated password will include uppercase alphabetic characters.

After generating a password, you will be taken to a second screen where you can choose to export your password history to JSON or CSV format. Use the arrow keys to select your desired option and press "Enter" to confirm. The exported file will be saved in your home directory.

To copy the generated password to your clipboard, simply press `Ctrl+C` in the second screen. The password will be copied, and you can paste it wherever needed.
