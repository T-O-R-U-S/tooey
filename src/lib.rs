#![feature(generic_arg_infer)]
#![feature(int_roundings)]
#![cfg_attr(not(test), no_std)]

pub mod objects;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::types::Terminal;

    use super::types::TerminalObject;

    #[test]
    fn prompt() {
        let mut terminal: Terminal<128, 128, char> = Terminal::new();

        let prompt: TerminalObject<128, 128, char> =
            TerminalObject::prompt(&"Wassuhhh dude, how you doinggggggg???");

        terminal.insert_object(prompt, 1).ok();

        terminal.frame();
    }
}
