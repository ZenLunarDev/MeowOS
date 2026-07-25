use crate::framebuffer::FrameBuffer;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(dead_code)]
impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn white() -> Self {
        Self::rgb(255, 255, 255)
    }

    pub const fn gray() -> Self {
        Self::rgb(128, 128, 128)
    }

    pub const fn dark_gray() -> Self {
        Self::rgb(64, 64, 64)
    }

    pub const fn green() -> Self {
        Self::rgb(0, 180, 0)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Widget {
    Button {
        rect: Rect,
        label: &'static str,
        pressed: bool,
    },
    Checkbox {
        rect: Rect,
        label: &'static str,
        checked: bool,
    },
    ProgressBar {
        rect: Rect,
        percent: u8,
    },
}

impl Widget {
    pub fn draw(&self, fb: &mut FrameBuffer) {
        match self {
            Widget::Button { rect, label, pressed } => {
                let bg = if *pressed { Color::dark_gray() } else { Color::gray() };
                let border = Color::white();
                fb.draw_rect(rect.x, rect.y, rect.w, rect.h, bg.r, bg.g, bg.b);
                for dy in 0..rect.h {
                    for dx in 0..rect.w {
                        if dx == 0 || dy == 0 || dx == rect.w - 1 || dy == rect.h - 1 {
                            fb.draw_pixel(rect.x + dx, rect.y + dy, border.r, border.g, border.b);
                        }
                    }
                }
                let _ = fb.draw_text(rect.x + 4, rect.y + 4, label, 255, 255, 255);
            }
            Widget::Checkbox { rect, label, checked } => {
                let box_color = if *checked { Color::green() } else { Color::dark_gray() };
                fb.draw_rect(rect.x, rect.y, 12, 12, box_color.r, box_color.g, box_color.b);
                fb.draw_rect(rect.x, rect.y, 12, 1, 255, 255, 255);
                fb.draw_rect(rect.x, rect.y, 1, 12, 255, 255, 255);
                fb.draw_rect(rect.x + 11, rect.y, 1, 12, 255, 255, 255);
                fb.draw_rect(rect.x, rect.y + 11, 12, 1, 255, 255, 255);
                let _ = fb.draw_text(rect.x + 16, rect.y + 2, label, 255, 255, 255);
            }
            Widget::ProgressBar { rect, percent } => {
                fb.draw_rect(rect.x, rect.y, rect.w, rect.h, 40, 40, 40);
                let fill = (rect.w as u32 * (*percent as u32)) / 100;
                if fill > 0 {
                    fb.draw_rect(rect.x, rect.y, fill as usize, rect.h, 0, 180, 0);
                }
                fb.draw_rect(rect.x, rect.y, rect.w, 1, 255, 255, 255);
                fb.draw_rect(rect.x, rect.y, 1, rect.h, 255, 255, 255);
                fb.draw_rect(rect.x + rect.w - 1, rect.y, 1, rect.h, 255, 255, 255);
                fb.draw_rect(rect.x, rect.y + rect.h - 1, rect.w, 1, 255, 255, 255);
            }
        }
    }

    #[allow(dead_code)]
    pub fn contains(&self, mx: usize, my: usize) -> bool {
        let rect = match self {
            Widget::Button { rect, .. } => rect,
            Widget::Checkbox { rect, .. } => rect,
            Widget::ProgressBar { rect, .. } => rect,
        };
        mx >= rect.x && mx < rect.x + rect.w && my >= rect.y && my < rect.y + rect.h
    }
}
