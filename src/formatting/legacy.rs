use crossterm::style::{Color, Stylize};

pub fn format(input: &str) -> Vec<String> {
    let mut lines = vec![];
    let mut line = String::new();
    let mut chars = input.chars().peekable();

    let mut fg: Option<Color> = None;
    let mut bold = false;
    let mut underline = false;

    while let Some(ch) = chars.next() {
        if ch == '§'
            && let Some(code) = chars.next()
        {
            match code {
                '0' => fg = Some(Color::Black),
                '1' => fg = Some(Color::DarkBlue),
                '2' => fg = Some(Color::DarkGreen),
                '3' => fg = Some(Color::DarkCyan),
                '4' => fg = Some(Color::DarkRed),
                '5' => fg = Some(Color::DarkMagenta),
                '6' => fg = Some(Color::DarkYellow),
                '7' => fg = Some(Color::Grey),
                '8' => fg = Some(Color::DarkGrey),
                '9' => fg = Some(Color::Blue),
                'a' => fg = Some(Color::Green),
                'b' => fg = Some(Color::Cyan),
                'c' => fg = Some(Color::Red),
                'd' => fg = Some(Color::Magenta),
                'e' => fg = Some(Color::Yellow),
                'f' => fg = Some(Color::White),
                'l' => bold = true,
                'n' => underline = true,
                'r' => {
                    fg = None;
                    bold = false;
                    underline = false;
                }
                _ => {
                    line.push('§');
                    line.push(code);
                }
            }
        } else if ch == '\n' {
            lines.push(line.clone());
            line.clear();
        } else {
            let mut styled = ch.to_string();
            if let Some(color) = fg {
                styled = styled.with(color).to_string();
            }
            if bold {
                styled = styled.bold().to_string();
            }
            if underline {
                styled = styled.underlined().to_string();
            }
            line.push_str(&styled);
        }
    }

    if !line.is_empty() {
        lines.push(line);
    }
    lines
}
