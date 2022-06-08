pub mod input;
pub mod controller;

use winit::event::{WindowEvent, KeyboardInput, VirtualKeyCode, ElementState};
    
pub fn try_get_key(event: &WindowEvent) -> (bool, Option<VirtualKeyCode>, Option<ElementState>) {
    match event {
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    virtual_keycode: Some(key),
                    state,
                    ..
                },
            ..
        } => (true, Some(*key), Some(*state)),
        _ => (false, None, None),
    }
}