use crossterm::style::{Color, Stylize};

pub fn format(width: usize, input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars();
    let mut column = 0;

    let mut fg: Option<Color> = None;
    let mut bold = false;
    let mut underline = false;

    while let Some(char) = chars.next() {
        if char == '§'
            && let Some(code) = chars.next()
            && column < width - 3
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
                    output.push('§');
                    output.push(code);
                    column += 2;
                }
            }
        } else if char == '\n' {
            if column == width - 3 {
                output += "...";
            }
            output += "\n";
            column = 0;
        } else if column < width - 3 {
            column += 1;
            let mut styled = char.to_string();
            if let Some(color) = fg {
                styled = styled.with(color).to_string();
            }
            if bold {
                styled = styled.bold().to_string();
            }
            if underline {
                styled = styled.underlined().to_string();
            }
            output += &styled;
        }
    }

    if column == width - 3 {
        output += "...";
    }
    output
}
