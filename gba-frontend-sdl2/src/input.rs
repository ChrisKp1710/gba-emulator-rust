// Input handling placeholder
// TODO: Implementare mappatura tasti GBA

use sdl2::keyboard::Keycode;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum GbaButton {
    A,
    B,
    L,
    R,
    Start,
    Select,
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
pub struct InputMapper {
    // TODO: Implementazione mappatura input
}

#[allow(dead_code)]
impl InputMapper {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn map_key(&self, key: Keycode) -> Option<GbaButton> {
        match key {
            Keycode::Z => Some(GbaButton::A),
            Keycode::X => Some(GbaButton::B),
            Keycode::A => Some(GbaButton::L),
            Keycode::S => Some(GbaButton::R),
            Keycode::Return => Some(GbaButton::Start),
            Keycode::Backspace => Some(GbaButton::Select),
            Keycode::Up => Some(GbaButton::Up),
            Keycode::Down => Some(GbaButton::Down),
            Keycode::Left => Some(GbaButton::Left),
            Keycode::Right => Some(GbaButton::Right),
            _ => None,
        }
    }
}

impl Default for InputMapper {
    fn default() -> Self {
        Self::new()
    }
}
