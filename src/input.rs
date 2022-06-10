pub mod input;
pub mod controller;

use winit::event::{WindowEvent, KeyboardInput, VirtualKeyCode, ElementState};
use crate::math::vector;

pub struct Input {
    pub mouse_position: vector::Vector2,
    pub move_up: bool,
    pub move_down: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub action1: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_position: vector::Vector2::zero(),
            move_up: false,
            move_down: false,
            move_left: false,
            move_right: false,
            action1: false,
        }
    }
    
    pub fn process_input_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => {
                match Some(*key).unwrap() {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        if Some(*state).unwrap() == ElementState::Pressed {
                            self.move_up = true;
                        }
                        else {
                            self.move_up = false;
                        }
                        return true;
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        if Some(*state).unwrap() == ElementState::Pressed {
                            self.move_down = true;
                        }
                        else {
                            self.move_down = false;
                        }
                        return true;
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        if Some(*state).unwrap() == ElementState::Pressed {
                            self.move_left = true;
                        }
                        else {
                            self.move_left = false;
                        }
                        return true;
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        if Some(*state).unwrap() == ElementState::Pressed {
                            self.move_right = true;
                        }
                        else {
                            self.move_right = false;
                        }
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            },
            _ => {
                return false;
            }
        }
    }
    
    pub fn get_mouse_position(&self) -> vector::Vector2 {
        return self.mouse_position;
    }
}