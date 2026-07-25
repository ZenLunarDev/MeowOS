use uefi::boot;
use uefi::proto::console::gop::{GraphicsOutput, PixelFormat};
use uefi::Status;

#[derive(Debug, Clone, Copy)]
pub struct FrameBuffer {
    ptr: *mut u8,
    width: usize,
    height: usize,
    stride: usize,
    pixel_format: PixelFormat,
}

impl FrameBuffer {
    pub const fn empty() -> Self {
        Self {
            ptr: core::ptr::null_mut(),
            width: 0,
            height: 0,
            stride: 0,
            pixel_format: PixelFormat::Rgb,
        }
    }

    pub fn init(&mut self) -> Result<(), Status> {
        let gfx_handle = boot::get_handle_for_protocol::<GraphicsOutput>()
            .map_err(|_| Status::DEVICE_ERROR)?;
        let mut gfx = boot::open_protocol_exclusive::<GraphicsOutput>(gfx_handle)
            .map_err(|_| Status::DEVICE_ERROR)?;

        let mode_info = gfx.current_mode_info();
        let mut fb = gfx.frame_buffer();

        self.ptr = fb.as_mut_ptr();
        self.width = mode_info.resolution().0 as usize;
        self.height = mode_info.resolution().1 as usize;
        self.stride = mode_info.stride() as usize;
        self.pixel_format = mode_info.pixel_format();

        Ok(())
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn draw_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x >= self.width || y >= self.height || self.ptr.is_null() {
            return;
        }
        let offset = y * self.stride * 4 + x * 4;
        unsafe {
            match self.pixel_format {
                PixelFormat::Rgb => {
                    let p = self.ptr.add(offset);
                    p.write_volatile(r);
                    p.add(1).write_volatile(g);
                    p.add(2).write_volatile(b);
                    p.add(3).write_volatile(0);
                }
                PixelFormat::Bgr => {
                    let p = self.ptr.add(offset);
                    p.write_volatile(b);
                    p.add(1).write_volatile(g);
                    p.add(2).write_volatile(r);
                    p.add(3).write_volatile(0);
                }
                _ => {}
            }
        }
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, r: u8, g: u8, b: u8) {
        for dy in 0..h {
            for dx in 0..w {
                self.draw_pixel(x + dx, y + dy, r, g, b);
            }
        }
    }

    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.draw_pixel(x, y, r, g, b);
            }
        }
    }
}
