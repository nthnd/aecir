pub fn go_xy_relative(x: i8, y: i8) -> String {
    let x_axis = if x.is_positive() { "C" } else { "D" };
    let y_axis = if y.is_positive() { "B" } else { "A" };
    let x = x.abs();
    let y = y.abs();
    format!("\x1b[{x}{x_axis}\x1b[{y}{y_axis}")
}

pub fn go_xy_absolute(x: u16, y: u16) -> String {
    format!("\x1b[{y};{x}H")
}

pub const HOME: &str = "\x1b[H";
