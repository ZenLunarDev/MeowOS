extern crate alloc;
use alloc::string::String;

use crate::framebuffer::FrameBuffer;
use core::time::Duration;
use uefi::boot;
use uefi::proto::console::text::{Input, Key, Output};

const FG: u8 = 220;
const BG: u8 = 10;

pub struct TextShell<'a> {
    fb: &'a mut FrameBuffer,
    input: &'a mut Input,
    output: &'a mut Output,
    cursor_x: usize,
    cursor_y: usize,
    line_h: usize,
    char_w: usize,
}

impl<'a> TextShell<'a> {
    pub fn new(fb: &'a mut FrameBuffer, input: &'a mut Input, output: &'a mut Output) -> Self {
        Self {
            fb,
            input,
            output,
            cursor_x: 0,
            cursor_y: 0,
            line_h: 10,
            char_w: 8,
        }
    }

    pub fn init(&mut self) {
        self.fb.clear(BG, BG, BG);
        self.cursor_x = 4;
        self.cursor_y = 4;
    }

    pub fn draw_cursor(&mut self) {
        self.fb.draw_rect(self.cursor_x, self.cursor_y + 8, 8, 2, FG, FG, FG);
    }

    pub fn clear_cursor(&mut self) {
        self.fb.draw_rect(self.cursor_x, self.cursor_y + 8, 8, 2, BG, BG, BG);
    }

    pub fn write_char(&mut self, c: char) {
        self.clear_cursor();
        if c == '\n' {
            self.cursor_x = 4;
            self.cursor_y += self.line_h;
        } else {
            self.fb.draw_char(self.cursor_x, self.cursor_y, c, FG, FG, FG);
            self.cursor_x += self.char_w;
        }
        if self.cursor_x + self.char_w >= self.fb.width() {
            self.cursor_x = 4;
            self.cursor_y += self.line_h;
        }
        if self.cursor_y + self.line_h >= self.fb.height() {
            self.scroll_up();
        }
        self.draw_cursor();
    }

    pub fn backspace(&mut self) {
        self.clear_cursor();
        if self.cursor_x >= 4 + self.char_w {
            self.cursor_x -= self.char_w;
            self.fb.draw_rect(self.cursor_x, self.cursor_y, self.char_w, self.line_h, BG, BG, BG);
        }
        self.draw_cursor();
    }

    pub fn newline(&mut self) {
        self.clear_cursor();
        self.cursor_x = 4;
        self.cursor_y += self.line_h;
        if self.cursor_y + self.line_h >= self.fb.height() {
            self.scroll_up();
        }
        self.draw_cursor();
    }

    pub fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            if c == '\n' {
                self.newline();
            } else {
                self.write_char(c);
            }
        }
    }

    pub fn write_line(&mut self, s: &str) {
        self.write_str(s);
        self.newline();
    }

    fn scroll_up(&mut self) {
        let w = self.fb.width();
        let h = self.fb.height();
        for y in 0..h - self.line_h {
            for x in 0..w {
                let r = self.fb.read_pixel(x, y + self.line_h);
                self.fb.draw_pixel(x, y, r.0, r.1, r.2);
            }
        }
        for y in h - self.line_h..h {
            for x in 0..w {
                self.fb.draw_pixel(x, y, BG, BG, BG);
            }
        }
        self.cursor_y -= self.line_h;
    }

    pub fn read_line(&mut self) -> alloc::string::String {
        use alloc::string::String;
        let mut buf = String::new();
        loop {
            match self.input.read_key() {
                Ok(Some(Key::Printable(c))) => {
                    let ch: char = c.into();
                    if ch == '\r' {
                        self.newline();
                        break;
                    } else if ch == '\u{8}' {
                        if !buf.is_empty() {
                            buf.pop();
                            self.backspace();
                        }
                    } else if ch >= ' ' && ch <= '~' {
                        buf.push(ch);
                        self.write_char(ch);
                    }
                }
                Ok(Some(Key::Special(_))) => {}
                Ok(None) => {
                    boot::stall(Duration::from_millis(10));
                }
                Err(_) => {
                    self.write_line("[err] read");
                }
            }
        }
        buf
    }

    pub fn run(&mut self) -> ! {
        self.init();
        self.write_line("MeowOS Kernel v0.3");
        self.write_line("100% Rust | no underlying OS");
        self.write_line("Type 'help' for commands");
        self.newline();

        loop {
            self.write_str("mewos> ");
            let cmd = self.read_line();
            let cmd = cmd.trim();

            match cmd {
                "help" => {
                    self.write_line("commands:");
                    self.write_line("  help   - this message");
                    self.write_line("  rect   - draw random rects");
                    self.write_line("  clear  - clear screen");
                    self.write_line("  gui    - widget demo");
                    self.write_line("  mouse  - mouse status");
                    self.write_line("  shot   - save screenshot");
                    self.write_line("  exit   - halt");
                }
                "rect" => {
                    let mut rng = 1u32;
                    rng = (rng * 1103515245 + 12345) % (1 << 31);
                    let rx = (rng % (self.fb.width() as u32).max(300)) as usize;
                    let ry = ((rng >> 8) % (self.fb.height() as u32).max(200)) as usize;
                    let r_ = (rng & 0xFF) as u8;
                    let g_ = ((rng >> 8) & 0xFF) as u8;
                    let b_ = ((rng >> 16) & 0xFF) as u8;
                    self.fb.draw_rect(rx, ry, 60 + (rng % 140) as usize, 40 + (rng % 100) as usize, r_, g_, b_);
                    self.write_line("rect done");
                }
                "clear" => {
                    self.init();
                }
                "gui" => {
                    use crate::gui::Widget;
                    self.write_line("Widgets Demo:");
                    let widgets = [
                        Widget::Button { rect: crate::gui::Rect { x: 4, y: 40, w: 120, h: 30 }, label: "Submit", pressed: false },
                        Widget::Checkbox { rect: crate::gui::Rect { x: 140, y: 40, w: 120, h: 20 }, label: "Enable", checked: true },
                        Widget::ProgressBar { rect: crate::gui::Rect { x: 4, y: 80, w: 260, h: 20 }, percent: 65 },
                    ];
                    for w in &widgets {
                        w.draw(self.fb);
                        self.newline();
                    }
                }
                "mouse" => {
                    let _ = crate::mouse::init_mouse();
                    self.write_line("mouse init done");
                }
                "shot" => {
                    let _ = crate::screenshot::save_screenshot(self.fb, "shot.bmp");
                    self.write_line("screenshot done");
                }
                "exit" => {
                    self.write_line("halting...");
                    loop { boot::stall(Duration::from_secs(1)); }
                }
                "" => {}
                _ => {
                    self.write_line("unknown command");
                }
            }
        }
    }
}
