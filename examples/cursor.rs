use aecir::{buffer, cursor};
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

fn main() {
    let mut stdout = stdout();
    write!(stdout, "{}{}", buffer::ENTER_ALT, cursor::HOME).unwrap();
    stdout.flush().unwrap();

    write!(stdout, "{}I'm over here", cursor::go_xy(6, 18)).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_secs(1));

    write!(stdout, "{}I'm over there", cursor::go_xy(23, -5)).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_secs(1));

    write!(stdout, "{}And now I'm over here", cursor::go_xy(-25, 3)).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_secs(1));

    write!(stdout, "{}", buffer::EXIT_ALT).unwrap();
    stdout.flush().unwrap();
}
