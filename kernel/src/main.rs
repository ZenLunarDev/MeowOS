#![no_main]
#![no_std]

use log::info;
use uefi::{entry, prelude::*};

mod framebuffer;
mod shell;

use framebuffer::FrameBuffer;

static mut FB: FrameBuffer = FrameBuffer::empty();

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Hello from Rust OS!");
    unsafe {
        FB.init().expect("FB init failed");
        FB.draw_rect(100, 100, 200, 150, 0xFF, 0x33, 0x33);
        FB.draw_rect(330, 120, 200, 150, 0x33, 0xFF, 0x33);
        FB.draw_rect(560, 140, 200, 150, 0x33, 0x33, 0xFF);
        shell::run(&mut FB);
    }
    Status::SUCCESS
}
