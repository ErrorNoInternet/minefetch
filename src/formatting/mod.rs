pub mod component;
pub mod legacy;

#[derive(Clone, Copy)]
pub enum Pad {
    Left,
    Right,
}

pub fn pad<T: ToString>(text: &T, len: usize, width: usize, pad: Pad) -> String {
    let mut string = String::new();
    match pad {
        Pad::Left => {
            string.push_str(&text.to_string());
            string.push_str(&" ".repeat(width - len));
        }
        Pad::Right => {
            string.push_str(&" ".repeat(width - len));
            string.push_str(&text.to_string());
        }
    }
    string
}
