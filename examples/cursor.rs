use aecir::{
    buffer, cursor,
    style::{Color, ColorName},
};
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

fn main() {
    let mut stdout = stdout();

    // Enter alternate buffer
    write!(stdout, "{}{}", buffer::ENTER_ALT, cursor::HOME).unwrap();
    stdout.flush().unwrap();

    // Write to absolute positions
    for i in 0..5 {
        let message = "Absolute Positioning";
        write!(
            stdout,
            "{}{}{message}",
            Color::Fg(ColorName::Blue),
            cursor::go_xy_absolute(10 + i * 2, 15 + i)
        )
        .unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(100));
    }

    // Write to relative positions
    for _ in 0..5 {
        let message = "Relative Positioning";
        write!(
            stdout,
            "{}{}{message}",
            Color::Fg(ColorName::Red),
            cursor::go_xy_relative(-(message.len() as i8) + 2, 1)
        )
        .unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(100));
    }

    sleep(Duration::from_secs(1));

    // Exit Alternate buffer
    write!(stdout, "{}", buffer::EXIT_ALT).unwrap();
    stdout.flush().unwrap();
}
