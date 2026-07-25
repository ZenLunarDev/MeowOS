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
    info!("Hello, MeowOS!");

    unsafe {
        FB.init().expect("FB init failed");

        FB.draw_rect(0, 0, FB.width(), FB.height(), 30, 30, 30);

        let mut wm = WindowManager::new();
        let app1 = wm.add(60, 50, 340, 220, "MeowOS Terminal", window_manager::Color::rgb(40, 40, 45));
        let app2 = wm.add(420, 80, 340, 220, "Notepad", window_manager::Color::rgb(35, 35, 40));
        let app3 = wm.add(240, 220, 340, 220, "Settings", window_manager::Color::rgb(45, 35, 35));

        wm.draw_all(&mut FB);

        FB.draw_text(20, FB.height() - 30, "Taskbar: Start | Apps minimized: 0 | Clock: 00:00", 220, 220, 220);

        let stdin_handle = boot::get_handle_for_protocol::<uefi::proto::console::text::Input>().unwrap();
        let mut input = boot::open_protocol_exclusive::<uefi::proto::console::text::Input>(stdin_handle).unwrap();

        let mut shell = text_shell::TextShell::new(&mut FB, &mut input);
        shell.run();
    }
}
