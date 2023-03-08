use core::borrow::BorrowMut;

use crate::types::{ColourChar, LifecycleEvent, TerminalObject};

/// This object clears the screen entirely on every frame; this should always be placed at layer zero to ensure that it does not wipe
/// living objects.
pub fn screen_cleaner<
    'a,
    const WIDTH: usize,
    const HEIGHT: usize,
    CHARACTER: BorrowMut<ColourChar> + From<char> + Clone + PartialEq,
>() -> TerminalObject<'a, WIDTH, HEIGHT, CHARACTER> {
    TerminalObject {
        on_update: |_, _, _| LifecycleEvent::NoEvent,
        on_draw: |screen, _| {
            for i in screen {
                for j in i {
                    *j = CHARACTER::from(' ')
                }
            }
        },
        data: &(),
    }
}
