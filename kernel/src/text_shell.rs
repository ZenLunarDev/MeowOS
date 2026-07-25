extern crate alloc;
use alloc::string::String;

use crate::framebuffer::FrameBuffer;
use core::time::Duration;
use uefi::boot;
use uefi::proto::console::text::{Input, Key};

pub struct TextShell<'a> {
    fb: &'a mut FrameBuffer,
    input: &'a mut Input,
    cursor_x: usize,
    cursor_y: usize,
}

impl<'a> TextShell<'a> {
    pub const fn new(fb: &'a mut FrameBuffer, input: &'a mut Input) -> Self {
        Self { fb, input, cursor_x: 4, cursor_y: 4 }
    }

    pub fn init(&mut self) {
        self.fb.clear(10, 10, 10);
        self.cursor_x = 4;
        self.cursor_y = 4;
    }

    pub fn write_char(&mut self, c: char) {
        if c == '\n' {
            self.cursor_x = 4;
            self.cursor_y += 10;
        } else {
            self.fb.draw_char(self.cursor_x, self.cursor_y, c, 220, 220, 220);
            self.cursor_x += 8;
        }
        if self.cursor_x + 8 >= self.fb.width() {
            self.cursor_x = 4;
            self.cursor_y += 10;
        }
        if self.cursor_y + 10 >= self.fb.height() {
            self.scroll_up();
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_x >= 4 + 8 {
            self.cursor_x -= 8;
            self.fb.draw_rect(self.cursor_x, self.cursor_y, 8, 10, 10, 10, 10);
        }
    }

    pub fn newline(&mut self) {
        self.cursor_x = 4;
        self.cursor_y += 10;
        if self.cursor_y + 10 >= self.fb.height() {
            self.scroll_up();
        }
    }

    fn scroll_up(&mut self) {
        let w = self.fb.width();
        let h = self.fb.height();
        for y in 0..h - 10 {
            for x in 0..w {
                let (r, g, b) = self.fb.read_pixel(x, y + 10);
                self.fb.draw_pixel(x, y, r, g, b);
            }
        }
        for y in h - 10..h {
            for x in 0..w {
                self.fb.draw_pixel(x, y, 10, 10, 10);
            }
        }
        self.cursor_y -= 10;
    }

    pub fn read_line(&mut self) -> String {
        let mut buf = String::new();
        loop {
            match self.input.read_key() {
                Ok(Some(Key::Printable(c))) => {
                    let code: u16 = c.into();
                    let ch = Self::uefi_to_ascii(code);
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
                Ok(None) => boot::stall(Duration::from_millis(10)),
                Err(_) => boot::stall(Duration::from_millis(10)),
            }
        }
        buf
    }

    fn uefi_to_ascii(code: u16) -> char {
        match code {
            0x0D => '\r',
            0x08 => '\u{8}',
            0x20 => ' ',
            0x21..=0x7E => (code as u8) as char,
            _ if code >= 0x41 && code <= 0x5A => (code as u8) as char,
            _ if code >= 0x61 && code <= 0x7A => (code as u8) as char,
            _ if code >= 0x30 && code <= 0x39 => (code as u8) as char,
            _ => '\0',
        }
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
                    self.write_line("  mouse  - mouse init");
                    self.write_line("  shot   - screenshot");
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
                "clear" => self.init(),
                "gui" => {
                    self.write_line("Widgets Demo:");
                    let _ = self.fb.draw_char(4, 40, 'B', 255, 80, 80);
                    let _ = self.fb.draw_char(4, 58, 'C', 80, 255, 80);
                    let _ = self.fb.draw_char(4, 76, 'P', 80, 80, 255);
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
                _ => self.write_line("unknown command"),
            }
        }
    }

    fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }

    fn write_line(&mut self, s: &str) {
        self.write_str(s);
        self.newline();
    }
}
