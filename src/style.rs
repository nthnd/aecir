use std::fmt::Display;

pub fn reset_all() -> String {
    format!(
        "\x1b[{}m",
        (20..=29).map(|num| format!("{num};")).collect::<String>()
    )
}

pub fn reset_colors() -> String {
    format!("\x1b[{}m", 0)
}

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub enum ColorName {
    Black,
    LightBlack,
    Red,
    LightRed,
    Green,
    LightGreen,
    Yellow,
    LightYellow,
    Blue,
    LightBlue,
    Magenta,
    LightMagenta,
    Cyan,
    LightCyan,
    #[default]
    White,
    LightWhite,
}

impl ColorName {
    pub fn get_value(&self) -> u16 {
        match self {
            ColorName::Black => 30,
            ColorName::LightBlack => 90,
            ColorName::Red => 31,
            ColorName::LightRed => 91,
            ColorName::Green => 32,
            ColorName::LightGreen => 92,
            ColorName::Yellow => 33,
            ColorName::LightYellow => 93,
            ColorName::Blue => 34,
            ColorName::LightBlue => 94,
            ColorName::Magenta => 35,
            ColorName::LightMagenta => 95,
            ColorName::Cyan => 36,
            ColorName::LightCyan => 96,
            ColorName::White => 37,
            ColorName::LightWhite => 97,
        }
    }
}

pub enum Color {
    Fg(ColorName),
    Bg(ColorName),
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Fg(c) => c.get_value(),
            Self::Bg(c) => c.get_value() + 10,
        };
        write!(f, "\x1b[{}m", value)
    }
}

pub enum Format {
    Bold,
    Dim,
    Italic,
    Underline,
    Blinking,
    Inverse,
    Hidden,
    Strikethrough,
}

impl Format {
    fn get_value(&self) -> u16 {
        match self {
            Self::Bold => 1,
            Self::Dim => 2,
            Self::Italic => 3,
            Self::Underline => 4,
            Self::Blinking => 5,
            Self::Inverse => 7,
            Self::Hidden => 8,
            Self::Strikethrough => 9,
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}m", self.get_value())
    }
}

pub struct Style {
    fg: ColorName,
    bg: ColorName,
    formats: Vec<Format>,
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let foreground = Color::Fg(self.fg);
        let background = Color::Bg(self.bg);
        write!(
            f,
            "{foreground}{background}{}",
            self.formats
                .iter()
                .map(Format::to_string)
                .collect::<String>()
        )
    }
}
