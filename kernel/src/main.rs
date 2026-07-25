#![no_main]
#![no_std]

use log::info;
use uefi::{boot, entry, prelude::*};
use uefi::proto::console::text::{Input, Output};

mod allocator;
mod framebuffer;
mod gui;
mod mouse;
mod screenshot;
mod text_shell;

use framebuffer::FrameBuffer;

static mut FB: FrameBuffer = FrameBuffer::empty();

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Hello, MeowOS!");

    unsafe {
        FB.init().expect("FB init failed");

        FB.draw_rect(100, 100, 200, 150, 0xFF, 0x33, 0x33);
        FB.draw_rect(330, 120, 200, 150, 0x33, 0xFF, 0x33);
        FB.draw_rect(560, 140, 200, 150, 0x33, 0x33, 0xFF);

        FB.draw_text(50, 50, "MeowOS Kernel v0.3", 255, 255, 255);

        let stdout_handle = boot::get_handle_for_protocol::<Output>().unwrap();
        let mut output = boot::open_protocol_exclusive::<Output>(stdout_handle).unwrap();
        let stdin_handle = boot::get_handle_for_protocol::<Input>().unwrap();
        let mut input = boot::open_protocol_exclusive::<Input>(stdin_handle).unwrap();

        let mut shell = text_shell::TextShell::new(&mut FB, &mut input, &mut output);
        shell.run();
    }
    Status::SUCCESS
}
