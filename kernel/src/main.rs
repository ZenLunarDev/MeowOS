#![no_main]
#![no_std]

use log::info;
use uefi::{entry, prelude::*};

mod allocator;
mod framebuffer;
mod gui;
mod mouse;
mod screenshot;
mod shell;

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

        FB.draw_text(50, 50, "MeowOS Kernel v0.2", 255, 255, 255);

        let _ = mouse::init_mouse();

        let widgets = [
            gui::Widget::Button {
                rect: gui::Rect { x: 50, y: 300, w: 120, h: 30 },
                label: "Submit",
                pressed: false,
            },
            gui::Widget::Checkbox {
                rect: gui::Rect { x: 200, y: 300, w: 120, h: 20 },
                label: "Enable",
                checked: true,
            },
            gui::Widget::ProgressBar {
                rect: gui::Rect { x: 50, y: 350, w: 300, h: 20 },
                percent: 65,
            },
            gui::Widget::ProgressBar {
                rect: gui::Rect { x: 50, y: 380, w: 300, h: 20 },
                percent: 30,
            },
        ];

        for w in &widgets {
            w.draw(&mut FB);
        }

        FB.draw_text(50, 420, "Type 'help' for commands", 200, 200, 200);

        shell::run(&mut FB);
    }
    Status::SUCCESS
}
