# IronKey

IronKey is a Terminal User Interface (TUI) based password generator written in Rust. It leverages the power of Rust's performance and safety features to provide a fast and secure way to generate passwords.

## Features

- **TUI Interface**: IronKey uses a text-based user interface, making it lightweight and easy to use directly from the terminal.

- **Customizable Password Generation**: IronKey generates passwords based on user-selected options. You can choose the length and the types of characters to include in the password.

- **Clipboard Support**: IronKey allows users to copy the generated password directly to their clipboard for ease of use.

- **Password Saving**: IronKey saves the generated password to a file in the user's home directory for future reference.

## Installation

To install IronKey, you need to have `Rust` & `Git` installed on your machine. If you don't have them installled in your machine, you can install it from the [Rust](https://www.rust-lang.org/tools/install) & [Git](https://git-scm.com/downloads).

Once Rust & Git is installed, you can install IronKey by cloning the repository and building the project:

```sh
git clone https://git.kekma.tech/IronKey.git
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

Once you've set your desired options, press the "Enter" key to generate the password. The generated password will be displayed on the screen and copied to your clipboard. It will also be saved to a file in your home directory for future reference.
