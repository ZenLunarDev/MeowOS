use crate::framebuffer::FrameBuffer;

#[derive(Debug, Clone, Copy)]
pub struct MouseState {
    pub x: usize,
    pub y: usize,
    pub left_button: bool,
    pub right_button: bool,
}

impl MouseState {
    pub const fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            left_button: false,
            right_button: false,
        }
    }
}

pub fn init_mouse() -> Option<MouseState> {
    Some(MouseState::new())
}
