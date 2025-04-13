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
            string += &text.to_string();
            string += &" ".repeat(width.saturating_sub(len));
        }
        Pad::Right => {
            string += &" ".repeat(width.saturating_sub(len));
            string += &text.to_string();
        }
    }
    string
}
