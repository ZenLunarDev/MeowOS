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

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub const fn white() -> Self {
        Self::rgb(255, 255, 255)
    }
    pub const fn black() -> Self {
        Self::rgb(0, 0, 0)
    }
    pub const fn gray() -> Self {
        Self::rgb(128, 128, 128)
    }
    pub const fn dark_gray() -> Self {
        Self::rgb(64, 64, 64)
    }
    pub const fn blue() -> Self {
        Self::rgb(0, 120, 215)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Window {
    pub id: u32,
    pub rect: Rect,
    pub title: &'static str,
    pub focused: bool,
    pub color: Color,
}

impl Window {
    pub const fn new(id: u32, x: usize, y: usize, w: usize, h: usize, title: &'static str, color: Color) -> Self {
        Self {
            id,
            rect: Rect { x, y, w, h },
            title,
            focused: false,
            color,
        }
    }

    pub fn draw(&self, fb: &mut FrameBuffer) {
        let bg = if self.focused { self.color } else { Color::dark_gray() };
        let bar_h = 20;

        fb.draw_rect(self.rect.x, self.rect.y, self.rect.w, self.rect.h, bg.r, bg.g, bg.b);

        if self.focused {
            fb.draw_rect(self.rect.x, self.rect.y, self.rect.w, 1, Color::blue().r, Color::blue().g, Color::blue().b);
            fb.draw_rect(self.rect.x, self.rect.y + bar_h, self.rect.w, 1, 60, 60, 60);
        } else {
            fb.draw_rect(self.rect.x, self.rect.y, self.rect.w, 1, 80, 80, 80);
        }

        fb.draw_rect(self.rect.x, self.rect.y, 1, self.rect.h, 80, 80, 80);
        fb.draw_rect(self.rect.x + self.rect.w - 1, self.rect.y, 1, self.rect.h, 80, 80, 80);
        fb.draw_rect(self.rect.x, self.rect.y + self.rect.h - 1, self.rect.w, 1, 80, 80, 80);

        let _ = fb.draw_text(self.rect.x + 6, self.rect.y + 4, self.title, 255, 255, 255);

        let btn_x = self.rect.x + self.rect.w - 18;
        let btn_y = self.rect.y + 4;
        fb.draw_rect(btn_x, btn_y, 14, 14, 200, 50, 50);
        fb.draw_rect(btn_x, btn_y, 14, 1, 255, 255, 255);
        fb.draw_rect(btn_x, btn_y, 1, 14, 255, 255, 255);
        fb.draw_rect(btn_x + 13, btn_y, 1, 14, 255, 255, 255);
        fb.draw_rect(btn_x, btn_y + 13, 14, 1, 255, 255, 255);

        let cx = btn_x + 5;
        let cy = btn_y + 3;
        fb.draw_pixel(cx, cy, 255, 255, 255);
        fb.draw_pixel(cx + 1, cy + 1, 255, 255, 255);
        fb.draw_pixel(cx + 2, cy + 2, 255, 255, 255);
        fb.draw_pixel(cx + 2, cy + 3, 255, 255, 255);
        fb.draw_pixel(cx + 1, cy + 4, 255, 255, 255);
        fb.draw_pixel(cx, cy + 5, 255, 255, 255);
        fb.draw_pixel(cx + 3, cy + 4, 255, 255, 255);
        fb.draw_pixel(cx + 4, cy + 3, 255, 255, 255);
        fb.draw_pixel(cx + 5, cy + 2, 255, 255, 255);
        fb.draw_pixel(cx + 6, cy + 1, 255, 255, 255);
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.rect.x && x < self.rect.x + self.rect.w && y >= self.rect.y && y < self.rect.y + self.rect.h
    }

    pub fn close_button_hit(&self, x: usize, y: usize) -> bool {
        let btn_x = self.rect.x + self.rect.w - 18;
        let btn_y = self.rect.y + 4;
        x >= btn_x && x < btn_x + 14 && y >= btn_y && y < btn_y + 14
    }

    pub fn title_bar_hit(&self, x: usize, y: usize) -> bool {
        x >= self.rect.x && x < self.rect.x + self.rect.w && y >= self.rect.y && y < self.rect.y + 20
    }
}

pub struct WindowManager {
    pub windows: [Option<Window>; 8],
    pub next_id: u32,
    pub drag: Option<(u32, usize, usize, usize, usize)>,
}

impl WindowManager {
    pub const fn new() -> Self {
        Self {
            windows: [None, None, None, None, None, None, None, None],
            next_id: 1,
            drag: None,
        }
    }

    pub fn add(&mut self, x: usize, y: usize, w: usize, h: usize, title: &'static str, color: Color) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        for i in 0..self.windows.len() {
            if self.windows[i].is_none() {
                self.windows[i] = Some(Window::new(id, x, y, w, h, title, color));
                self.focus(id);
                return id;
            }
        }
        0
    }

    pub fn focus(&mut self, id: u32) {
        for w in &mut self.windows {
            if let Some(ref mut win) = w {
                win.focused = win.id == id;
            }
        }
    }

    pub fn close(&mut self, id: u32) {
        for w in &mut self.windows {
            if let Some(win) = w {
                if win.id == id {
                    *w = None;
                    break;
                }
            }
        }
    }

    pub fn draw_all(&self, fb: &mut FrameBuffer) {
        for w in &self.windows {
            if let Some(win) = w {
                win.draw(fb);
            }
        }
    }

    pub fn handle_click(&mut self, x: usize, y: usize) {
        for i in (0..self.windows.len()).rev() {
            if let Some(win) = self.windows[i] {
                if win.contains(x, y) {
                    self.focus(win.id);
                    if win.close_button_hit(x, y) {
                        self.close(win.id);
                        return;
                    }
                    self.drag = Some((win.id, x, y, win.rect.x, win.rect.y));
                    return;
                }
            }
        }
    }

    pub fn handle_drag(&mut self, x: usize, y: usize) {
        if let Some((id, start_x, start_y, orig_x, orig_y)) = self.drag {
            let dx = x.saturating_sub(start_x);
            let dy = y.saturating_sub(start_y);
            let new_x = orig_x + dx;
            let new_y = orig_y + dy;
            for w in &mut self.windows {
                if let Some(ref mut win) = w {
                    if win.id == id {
                        win.rect.x = new_x;
                        win.rect.y = new_y;
                        break;
                    }
                }
            }
        }
    }

    pub fn end_drag(&mut self) {
        self.drag = None;
    }
}
