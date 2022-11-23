use aecir::buffer;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    print!("{}", buffer::ENTER_ALT);
    println!("Hello from another dimension.");
    sleep(Duration::from_secs(3));
    print!("{}", buffer::EXIT_ALT);
    println!("Hello from this dimension.");
}
