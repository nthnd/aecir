use aecir::style;

#[test]
fn color_sanity() {
    assert_eq!(style::reset_colors(), "\x1b[0m");
    assert_eq!(
        style::Color::Fg(style::ColorName::Red).to_string(),
        "\x1b[31m"
    )
}
