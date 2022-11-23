pub fn go_xy(x: i8, y: i8) -> String {
    let x_axis = if x.is_positive() { "C" } else { "D" };
    let y_axis = if y.is_positive() { "B" } else { "A" };
    let x = x.abs();
    let y = y.abs();
    format!("\x1b[{x}{x_axis}\x1b[{y}{y_axis}")
}

pub const HOME: &str = "\x1b[H";
