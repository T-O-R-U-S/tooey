use core::borrow::BorrowMut;

use crate::types::{Colour, ColourChar, LifecycleEvent, Terminal, TerminalObject, TerminalUpdate};

/// This object clears the screen entirely on every frame; this should always be placed at layer zero to ensure that it does not wipe
/// living objects.
pub fn screen_cleaner<
    'a,
    const WIDTH: usize,
    const HEIGHT: usize,
    CHARACTER: BorrowMut<ColourChar> + From<ColourChar> + Clone,
>(
    bg_colour: &'a Colour,
) -> TerminalObject<'a, WIDTH, HEIGHT, CHARACTER> {
    TerminalObject {
        on_update: |_, _, update_payload| match update_payload {
            TerminalUpdate::Arbitrary("kill screen cleaner") => LifecycleEvent::Death,
            _ => LifecycleEvent::NoEvent,
        },
        on_draw: |screen, data| {
            let bg_colour: &Colour = data.downcast_ref().unwrap();

            for i in screen {
                for j in i {
                    *j = ColourChar::Colour(Colour::U8(0), *bg_colour, ' ').into()
                }
            }
        },
        data: bg_colour,
    }
}

pub fn colour_prompt<
    'a,
    const WIDTH: usize,
    const HEIGHT: usize,
    CHARACTER: BorrowMut<ColourChar> + From<ColourChar> + Clone,
>(
    data: &'a (Colour, Colour, &'static str),
) -> TerminalObject<'a, WIDTH, HEIGHT, CHARACTER> {
    TerminalObject {
        on_update: |_, _, update_payload| match update_payload {
            TerminalUpdate::KeyboardEvent(_) | TerminalUpdate::MouseClick(_, _) => {
                LifecycleEvent::Death
            }
            _ => LifecycleEvent::NoEvent,
        },
        on_draw: |screen, data| {
            let (fg_colour, bg_colour, text): &(Colour, Colour, &str) =
                data.downcast_ref().unwrap();

            let length = text.len();

            let horizontal_middle = WIDTH / 2;
            let vertical_middle = HEIGHT / 2;

            let lines = length.div_ceil(WIDTH);

            let box_x = horizontal_middle
                .checked_sub(length / 2)
                .map(|x| x.checked_sub(1).unwrap_or(0))
                .unwrap_or(0);
            let box_y = vertical_middle
                .checked_sub(lines / 2)
                .map(|x| x.checked_sub(1).unwrap_or(0))
                .unwrap_or(0);

            let mut box_width = horizontal_middle + (length / 2) + 1;
            let mut box_height = vertical_middle + (lines / 2) + 1;

            let mut overflow_flag = false;

            // Clamp box_height and box_width to prevent out-of-bounds index in case of large input.
            if box_width > WIDTH {
                box_height += box_width / WIDTH;
                box_width = WIDTH;
            }

            if box_height > HEIGHT {
                box_height = HEIGHT;
                overflow_flag = true;
            }

            let mut text = text.chars();

            // Print characters that fit in the box
            'a: for y in box_y..box_height {
                for x in box_x..box_width {
                    // If there are no more character to print, exit the loop
                    let Some(character) = text.next() else {
                        // Semi-rare loop labelling syntax: https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html
                        break 'a;
                    };
                    screen[y][x] = ColourChar::Colour(*fg_colour, *bg_colour, character).into()
                }
            }

            if overflow_flag {
                // change last three characters to "..." to represent the fact that info is incomplete
                screen[HEIGHT - 1][WIDTH - 1] =
                    ColourChar::Colour(*fg_colour, *bg_colour, '.').into();
                screen[HEIGHT - 1][WIDTH - 2] =
                    ColourChar::Colour(*fg_colour, *bg_colour, '.').into();
                screen[HEIGHT - 1][WIDTH - 3] =
                    ColourChar::Colour(*fg_colour, *bg_colour, '.').into();
            }
        },
        data,
    }
}

pub fn vertical_split<
    'a,
    const WIDTH: usize,
    const HEIGHT: usize,
    const X_SPLIT: usize,
    CHARACTER: BorrowMut<ColourChar> + From<ColourChar> + From<char> + Clone + 'a,
>(
    terminal: &mut Terminal<'a, WIDTH, HEIGHT, CHARACTER>,
) -> TerminalObject<'a, WIDTH, HEIGHT, CHARACTER>
where
    &'a mut CHARACTER: Clone + BorrowMut<ColourChar> + From<ColourChar> + Default + From<char>,
    [(); WIDTH - X_SPLIT]:,
{
    let first_half = &mut terminal.characters[..][0..X_SPLIT];
    let second_half = &mut terminal.characters[..][X_SPLIT..];

    let first_half_terminal: Terminal<X_SPLIT, HEIGHT, &mut CHARACTER> = Terminal::new();
    let second_half_terminal: Terminal<{ WIDTH - X_SPLIT }, HEIGHT, &mut CHARACTER> =
        Terminal::new();

    TerminalObject::empty()
}
