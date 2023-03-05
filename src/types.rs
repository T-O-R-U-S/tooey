use core::{any::Any, borrow::BorrowMut};
use keyboard_types::KeyboardEvent;

pub struct Terminal<'a, const WIDTH: usize, const HEIGHT: usize, CHARACTER: BorrowMut<char>> {
    characters: [[CHARACTER; WIDTH]; HEIGHT],
    objects: [TerminalObject<'a, WIDTH, HEIGHT, CHARACTER>; 256],
}

pub struct TerminalObject<'a, const WIDTH: usize, const HEIGHT: usize, CHARACTER: BorrowMut<char>> {
    pub on_update: fn(&mut [[CHARACTER; WIDTH]; HEIGHT], &dyn Any, &TerminalUpdate),
    pub on_draw: fn(&mut [[CHARACTER; WIDTH]; HEIGHT], &dyn Any),
    pub data: &'a dyn Any,
}

#[derive(Clone)]
pub enum TerminalUpdate {
    KeyboardEvent(KeyboardEvent),
    MouseClick(usize, usize),
    Arbitrary(&'static str),
}

impl<'a, const WIDTH: usize, const HEIGHT: usize, CHARACTER: BorrowMut<char>>
    Terminal<'a, WIDTH, HEIGHT, CHARACTER>
{
    pub fn frame(&mut self) {
        for object in &self.objects {
            (object.on_draw)(&mut self.characters, object.data)
        }
    }

    pub fn update(&mut self, update_payload: TerminalUpdate) {
        for object in &self.objects {
            (object.on_update)(&mut self.characters, object.data, &update_payload)
        }
    }

    pub fn insert_object(
        &mut self,
        object: TerminalObject<'a, WIDTH, HEIGHT, CHARACTER>,
    ) -> Result<(), TerminalObject<'a, WIDTH, HEIGHT, CHARACTER>> {
        Ok(())
    }
}
