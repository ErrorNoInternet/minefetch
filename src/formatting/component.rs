use crate::formatting::legacy;
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

pub fn format(width: usize, component: &TextComponent) -> String {
    let mut output = String::new();
    format_component(width, component, &mut output);
    output
}

fn format_component(width: usize, component: &TextComponent, output: &mut String) {
    macro_rules! append {
        ($command:expr) => {{
            let _ = write!(output, "{}", $command);
        }};
    }

    if let Some(color) = component.color.as_deref().and_then(get_color) {
        append!(SetForegroundColor(color));
    }
    if component.bold {
        append!(SetAttribute(Attribute::Bold));
    }
    if component.italic {
        append!(SetAttribute(Attribute::Italic));
    }
    if component.underlined {
        append!(SetAttribute(Attribute::Underlined));
    }
    if component.strikethrough {
        append!(SetAttribute(Attribute::CrossedOut));
    }
    output.push_str(&legacy::format(width, &component.text));
    append!(SetAttribute(Attribute::Reset));
    append!(SetForegroundColor(Color::Reset));

    for extra in &component.extra {
        format_component(width, extra, output);
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
