#![feature(generic_arg_infer)]
#![feature(int_roundings)]
#![cfg_attr(
    not(any(test, target_family = "unix", target_family = "windows")),
    no_std
)]

pub mod objects;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::types::{ColourChar, Terminal};

    use super::types::TerminalObject;

    #[test]
    fn prompt() {
        let mut terminal: Terminal<128, 128, ColourChar> = Terminal::new();

        // This is peak testing right here.
        let prompt: TerminalObject<128, 128, ColourChar> =
            TerminalObject::prompt(&"Wassuhhh dude, how you doinggggggg???");

        terminal.insert_object(prompt, 1).ok();

        terminal.frame();
    }
}
