#![feature(generic_arg_infer)]

use tooey::objects::screen_cleaner;
use tooey::types::Terminal;
use tooey::types::TerminalObject;
use tooey::types::TerminalUpdate;

fn main() {
    let mut terminal: Terminal<64, 32, char> = Terminal::new();

    let prompt: TerminalObject<_, _, _> = TerminalObject::prompt(&"Kachow!");

    terminal.insert_object(screen_cleaner(), 0).ok();

    terminal.insert_object(prompt, 1).ok();

    print!("\u{001b}[42m");

    terminal.frame();

    for i in terminal.characters {
        for j in i {
            print!("{j}");
        }
        print!("\u{001b}[40m");
        println!();
        print!("\u{001b}[42m");
    }
    print!("\u{001b}[40m");

    let prompt: TerminalObject<_, _, _> =
        TerminalObject::prompt(&"This prompt should overwrite the last one!!");

    terminal.insert_object(prompt, 2).ok();

    terminal.frame();

    for i in terminal.characters {
        for j in i {
            print!("{j}");
        }
        print!("\u{001b}[40m");
        println!();
        print!("\u{001b}[42m");
    }
    print!("\u{001b}[40m");

    terminal.update(TerminalUpdate::Ping);

    terminal.frame();

    for i in terminal.characters {
        for j in i {
            print!("{j}");
        }
        print!("\u{001b}[40m");
        println!();
        print!("\u{001b}[42m");
    }
    print!("\u{001b}[40m");
}
