// This file is just used as a crude way to test.
// Actual tests should be written in `lib.rs`, under the `tests` module.

#![feature(generic_arg_infer)]

use tooey::objects::colour_prompt;
use tooey::objects::screen_cleaner;
use tooey::types::Colour;
use tooey::types::ColourChar;
use tooey::types::Terminal;
use tooey::types::TerminalObject;
use tooey::types::TerminalUpdate;

fn main() {
    let mut terminal: Terminal<64, 32, ColourChar> = Terminal::new();

    let prompt = TerminalObject::prompt(&"Kachow!");

    terminal
        .insert_object(screen_cleaner(&Colour::U8(128)), 0)
        .ok();

    terminal.insert_object(prompt, 1).ok();

    terminal.frame();

    for i in terminal.characters {
        for j in i {
            print!("{j}");
        }
        println!();
    }
    println!();

    let colour_prompt = colour_prompt(&(
        Colour::Rgb(255, 255, 255),
        Colour::Rgb(0, 128, 0),
        "This prompt should overwrite the last one!",
    ));

    //let prompt = TerminalObject::prompt(&"This prompt should cover the last one!!");

    terminal.insert_object(colour_prompt, 2).ok();

    terminal.frame();

    for i in terminal.characters {
        for j in i {
            print!("{j}");
        }
        println!()
    }

    println!();

    terminal.update(TerminalUpdate::MouseClick(0, 0));

    terminal.frame();

    for i in terminal.characters {
        for j in i {
            print!("{j}");
        }
        println!();
    }
}
