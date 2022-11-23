# aecir
Ansi escape codes in Rust.

## What is it?
Just some simple Rust aliases for ANSI codes.  
Here's an excerpt from a style example.
```rust
use aecir::style::{self, Color, ColorName, Format};

fn main() {
    println!("{}I am red text.", Color::Fg(ColorName::Red));
    println!(
        "{}Now, my background is blue.{}",
        Color::Bg(ColorName::Blue),
        style::reset_colors(),    
    );
    println!("{}Now, I am just now bold", Format::Bold);
    println!("{}... and italic", Format::Italic);
    println!("{}... and also yellow.", Color::Fg(ColorName::Yellow));
    println!("{}Now I am back to normal.", style::reset_all());
}
```
![2022-11-23-15:18:45-screenshot-cropped](https://user-images.githubusercontent.com/96471299/203545731-cb714f32-7307-4a5a-9b67-3c499e27e008.png)



