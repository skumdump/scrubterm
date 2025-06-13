use std::io::{Write, stdout};

fn main() {
    let mut stdout = stdout();

    // Clear visible screen
    write!(stdout, "\x1B[2J").unwrap();

    // Move cursor to top-left
    write!(stdout, "\x1B[H").unwrap();

    // Clear scrollback buffer
    write!(stdout, "\x1B[3J").unwrap();

    stdout.flush().unwrap();
}
