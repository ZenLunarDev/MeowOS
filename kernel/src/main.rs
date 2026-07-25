#![no_main]
#![no_std]

use log::info;
use uefi::{boot, entry, prelude::*};

mod allocator;
mod framebuffer;
mod gui;
mod mouse;
mod screenshot;
mod text_shell;
mod window_manager;

use framebuffer::FrameBuffer;
use window_manager::WindowManager;

static mut FB: FrameBuffer = FrameBuffer::empty();

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    
    unsafe {
        FB.init().expect("FB init failed");
        
        // ทดสอบวาดทั้งหมด
        FB.draw_rect(0, 0, FB.width(), FB.height(), 30, 30, 30);
        FB.draw_text(50, 50, "MeowOS Kernel v0.3", 255, 255, 255);
        FB.draw_text(50, 80, "100% Rust | no underlying OS", 255, 255, 255);
        FB.draw_rect(100, 100, 200, 150, 0xFF, 0x33, 0x33);
        FB.draw_rect(330, 120, 200, 150, 0x33, 0xFF, 0x33);
        FB.draw_rect(560, 140, 200, 150, 0x33, 0x33, 0xFF);
        
        // หยุด 15 วินาที
        for _ in 0..15000000 {
            core::hint::spin_loop();
        }
    }
    
    loop {}
}
