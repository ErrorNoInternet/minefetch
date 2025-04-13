use crossterm::style::{Attribute, Color, SetAttribute, SetForegroundColor};
use serde::Deserialize;
use std::fmt::Write;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Deserialize)]
pub struct TextComponent {
    pub text: String,
    #[serde(default)]
    pub extra: Vec<TextComponent>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underlined: bool,
    #[serde(default)]
    pub strikethrough: bool,
}

pub fn format(component: &TextComponent) -> Vec<String> {
    let mut output = String::new();
    format_component(&mut output, component);
    output.lines().map(ToOwned::to_owned).collect()
}

fn format_component(output: &mut String, component: &TextComponent) {
    if let Some(color) = component.color.as_deref().and_then(get_color) {
        let _ = write!(output, "{}", SetForegroundColor(color));
    }

    if component.bold {
        let _ = write!(output, "{}", SetAttribute(Attribute::Bold));
    }
    if component.italic {
        let _ = write!(output, "{}", SetAttribute(Attribute::Italic));
    }
    if component.underlined {
        let _ = write!(output, "{}", SetAttribute(Attribute::Underlined));
    }
    if component.strikethrough {
        let _ = write!(output, "{}", SetAttribute(Attribute::CrossedOut));
    }

    output.push_str(&component.text);
    let _ = write!(
        output,
        "{}{}",
        SetAttribute(Attribute::Reset),
        SetForegroundColor(Color::Reset)
    );

    for extra in &component.extra {
        format_component(output, extra);
    }
}

fn get_color(color: &str) -> Option<Color> {
    match color.to_lowercase().as_str() {
        "black" => Some(Color::Black),
        "dark_blue" => Some(Color::DarkBlue),
        "dark_green" => Some(Color::DarkGreen),
        "dark_aqua" | "dark_cyan" => Some(Color::DarkCyan),
        "dark_red" => Some(Color::DarkRed),
        "dark_purple" | "dark_magenta" => Some(Color::DarkMagenta),
        "gold" | "dark_yellow" => Some(Color::DarkYellow),
        "gray" | "grey" => Some(Color::Grey),
        "dark_gray" => Some(Color::DarkGrey),
        "blue" => Some(Color::Blue),
        "green" => Some(Color::Green),
        "aqua" | "cyan" => Some(Color::Cyan),
        "red" => Some(Color::Red),
        "light_purple" | "magenta" => Some(Color::Magenta),
        "yellow" => Some(Color::Yellow),
        "white" => Some(Color::White),
        _ => None,
    }
}
