pub mod component;
pub mod legacy;

use std::time::Duration;

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

pub const fn latency_bar(duration: Duration) -> char {
    match duration.as_millis() {
        0..=150 => '█',
        151..=300 => '▆',
        301..=450 => '▄',
        451..=600 => '▂',
        601.. => '▁',
    }
}
