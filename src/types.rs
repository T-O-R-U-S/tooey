use core::{any::Any, borrow::BorrowMut, fmt::Display};
use keyboard_types::KeyboardEvent;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Colour {
    U8(u8),
    Rgb(u8, u8, u8),
    #[default]
    Default,
}

#[cfg(any(
    target_os = "windows",
    target_os = "darwin",
    target_os = "posix",
    target_family = "windows",
    target_family = "unix"
))]
impl Display for Colour {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            return match self {
                Colour::U8(code) => write!(f, "\x1b[48;5;{code}m"),
                Colour::Rgb(r, g, b) => write!(f, "\x1b[48;2;{r};{g};{b}m"),
                Colour::Default => write!(f, "\x1b[49m"),
            };
        }
        match self {
            Colour::U8(code) => write!(f, "\x1b[38;5;{code}m"),
            Colour::Rgb(r, g, b) => write!(f, "\x1b[38;2;{r};{g};{b}m"),
            Colour::Default => write!(f, "\x1b[49m"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
// TODO: Integrate ColourChar into the library to support colours!
pub enum ColourChar {
    Colour(Colour, Colour, char),
    Monochrome(char),
    #[default]
    Empty,
}

#[cfg(any(
    target_os = "windows",
    target_os = "darwin",
    target_os = "posix",
    target_family = "windows",
    target_family = "unix"
))]
impl Display for ColourChar {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ColourChar::Colour(fg_colour, bg_colour, character) => {
                write!(f, "{bg_colour:#}{fg_colour}{character}\x1b[49m")
            }
            ColourChar::Monochrome(character) => write!(f, "{character}\x1b[49m"),
            ColourChar::Empty => write!(f, " "),
        }
    }
}

impl From<ColourChar> for char {
    fn from(value: ColourChar) -> Self {
        match value {
            ColourChar::Colour(_, _, character) => character,
            ColourChar::Monochrome(character) => character,
            ColourChar::Empty => ' ',
        }
    }
}

impl From<char> for ColourChar {
    fn from(value: char) -> Self {
        ColourChar::Monochrome(value)
    }
}

#[derive(Clone)]
pub struct Terminal<
    'a,
    const WIDTH: usize,
    const HEIGHT: usize,
    CHARACTER: BorrowMut<ColourChar> + Clone,
> {
    pub characters: [[CHARACTER; WIDTH]; HEIGHT],
    objects: [TerminalObject<'a, WIDTH, HEIGHT, CHARACTER>; 256],
}

#[derive(Clone, Copy)]
pub struct TerminalObject<
    'a,
    const WIDTH: usize,
    const HEIGHT: usize,
    CHARACTER: BorrowMut<ColourChar> + Clone,
> {
    pub on_update:
        fn(&mut [[CHARACTER; WIDTH]; HEIGHT], &dyn Any, &TerminalUpdate) -> LifecycleEvent,
    pub on_draw: fn(&mut [[CHARACTER; WIDTH]; HEIGHT], &dyn Any),
    pub data: &'a dyn Any,
}

/// This is returned at the end of every update to indicate to the virtual terminal
/// whether an object
pub enum LifecycleEvent {
    NoEvent,
    Death,
}

impl<
        'a,
        const WIDTH: usize,
        const HEIGHT: usize,
        CHARACTER: BorrowMut<ColourChar> + Clone + From<ColourChar> + From<char>,
    > TerminalObject<'a, WIDTH, HEIGHT, CHARACTER>
{
    fn prompt_on_update(
        _screen: &mut [[CHARACTER; WIDTH]; HEIGHT],
        _data: &dyn Any,
        update_payload: &TerminalUpdate,
    ) -> LifecycleEvent {
        match update_payload {
            TerminalUpdate::KeyboardEvent(_) | TerminalUpdate::MouseClick(_, _) => {
                LifecycleEvent::Death
            }
            _ => LifecycleEvent::NoEvent,
        }
    }

    fn prompt_on_draw(screen: &mut [[CHARACTER; WIDTH]; HEIGHT], data: &dyn Any) {
        let text: &&str = data.downcast_ref().unwrap();

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

        if box_width > WIDTH {
            box_height += box_width / WIDTH;
            box_width = WIDTH;
        }

        if box_height > HEIGHT {
            box_height = HEIGHT;
            overflow_flag = true;
        }

        let mut text = text.chars();

        for y in box_y..box_height {
            for x in box_x..box_width {
                let Some(character) = text.next() else {
                    continue
                };
                screen[y][x] = CHARACTER::from(character);
            }
        }

        if overflow_flag {
            // change last three characters to "..." to represent missing info
            screen[HEIGHT - 1][WIDTH - 1] = '.'.into();
            screen[HEIGHT - 1][WIDTH - 2] = '.'.into();
            screen[HEIGHT - 1][WIDTH - 3] = '.'.into();
        }
    }

    pub fn prompt(text: &'a dyn Any) -> Self {
        Self {
            on_update: Self::prompt_on_update,
            on_draw: Self::prompt_on_draw,
            data: text,
        }
    }

    pub fn empty() -> Self {
        Self {
            on_update: |_, _, _| LifecycleEvent::NoEvent,
            on_draw: |_, _| {},
            data: &(),
        }
    }
}

#[derive(Clone)]
pub enum TerminalUpdate {
    KeyboardEvent(KeyboardEvent),
    MouseClick(usize, usize),
    Arbitrary(&'static str),
    Ping,
    ForceUpdate,
}

impl<
        'a,
        const WIDTH: usize,
        const HEIGHT: usize,
        CHARACTER: BorrowMut<ColourChar> + Clone + From<ColourChar> + From<char>,
    > Terminal<'a, WIDTH, HEIGHT, CHARACTER>
{
    pub fn new() -> Self
    where
        CHARACTER: Default + Copy + From<char>,
    {
        Self {
            characters: [[CHARACTER::from(' '); WIDTH]; HEIGHT],
            objects: [TerminalObject::empty(); 256],
        }
    }

    pub fn frame(&mut self) {
        for object in &self.objects {
            (object.on_draw)(&mut self.characters, object.data)
        }
    }

    pub fn update(&mut self, update_payload: TerminalUpdate) {
        for object in self.objects.iter_mut() {
            match (object.on_update)(&mut self.characters, object.data, &update_payload) {
                LifecycleEvent::NoEvent => {}
                LifecycleEvent::Death => {
                    *object = TerminalObject::empty();
                }
            }
        }
    }

    pub fn insert_object(
        &mut self,
        object: TerminalObject<'a, WIDTH, HEIGHT, CHARACTER>,
        layer: usize,
    ) -> Result<(), TerminalObject<'a, WIDTH, HEIGHT, CHARACTER>> {
        if layer >= self.objects.len() {
            return Err(object);
        }

        self.objects[layer] = object;
        Ok(())
    }
}
