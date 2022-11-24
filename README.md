# aecir
Ansi escape codes in Rust.

## Usage
Add this your `Cargo.toml`
```toml
[dependencies]
aecir = {git='https://github.com/nate-sys/aecir', branch = 'main'}
```

## Examples
You can run the examples from this repo. 
```sh
git clone https://github.com/nate-sys/aecir
cd aecir
cargo run --example <example_name>
```
## Philosophy
Most of these aliases are just Structs and Enums that `impl std::fmt::Display`
or functions that return their corresponding string values. Almost all of 
these aliases expect _you_ to do the writing to `stdout` and flushing.  
  
```rust
use aecir::style::{self, Color, ColorName};
use std::io::{stdout, Write};

fn main() {
    let mut stdout = stdout(); 

    write!(
        stdout, 
        "{}Hello world{}", 
        Color::Fg(ColorName::Red), 
        style::reset_colors(),
    ).unwrap();

    stdout.flush().unwrap()
}
```


