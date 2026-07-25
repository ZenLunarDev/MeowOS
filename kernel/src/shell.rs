use crate::framebuffer::FrameBuffer;
use crate::gui::Widget;
use core::fmt::Write;
use core::time::Duration;
use uefi::{boot};
use uefi::proto::console::text::{Input, Key, Output};

fn shell_loop(
    output: &mut Output,
    input: &mut Input,
    fb: &mut FrameBuffer,
) -> ! {
    let mut buf = [0u16; 256];
    let mut rect_count: u32 = 0;

    let _ = output.reset(false);
    writeln!(output, "MewoOS Kernel v0.2").ok();
    writeln!(output, "100% Rust | no underlying OS").ok();
    writeln!(output, "Type 'help' for commands\r\n").ok();

    loop {
        write!(output, "mewos> ").ok();
        let mut pos = 0;

        while pos < buf.len() {
            match input.read_key() {
                Ok(Some(Key::Printable(c))) => {
                    let v: u16 = c.into();
                    let ch = char::from_u32(v as u32).unwrap_or('\0');
                    if ch == '\r' {
                        writeln!(output).ok();
                        break;
                    } else if ch == '\u{8}' {
                        if pos > 0 {
                            pos -= 1;
                            write!(output, "\u{8} \u{8}").ok();
                        }
                    } else if ch >= ' ' && ch <= '~' {
                        buf[pos] = ch as u16;
                        pos += 1;
                        write!(output, "{}", ch).ok();
                    }
                }
                Ok(Some(Key::Special(_))) => {}
                Ok(None) => {
                    boot::stall(Duration::from_millis(10));
                }
                Err(_) => {
                    writeln!(output, "[err] read error").ok();
                    continue;
                }
            }
        }

        let cmd = if pos == 0 {
            ""
        } else {
            let cmd_bytes = unsafe { core::slice::from_raw_parts(buf[..pos].as_ptr() as *const u8, pos) };
            core::str::from_utf8(cmd_bytes).map(|s| s.trim()).unwrap_or("")
        };

        match cmd {
            "help" => {
                writeln!(output, "  commands:").ok();
                writeln!(output, "    help   - this message").ok();
                writeln!(output, "    rect   - draw random rects").ok();
                writeln!(output, "    clear  - clear text console").ok();
                writeln!(output, "    cls    - clear framebuffer").ok();
                writeln!(output, "    gui    - show widget demo").ok();
                writeln!(output, "    mouse  - mouse status").ok();
                writeln!(output, "    shot   - save screenshot").ok();
                writeln!(output, "    exit   - halt").ok();
            }
            "rect" => {
                rect_count = rect_count.wrapping_add(1);
                let rx = (rect_count * 73) % (fb.width() as u32).max(300);
                let ry = (rect_count * 137) % (fb.height() as u32).max(200);
                let r_ = (rect_count & 0xFF) as u8;
                let g_ = ((rect_count >> 4) & 0xFF) as u8;
                let b_ = ((rect_count >> 8) & 0xFF) as u8;
                fb.draw_rect(
                    rx as usize,
                    ry as usize,
                    40 + ((rect_count * 47) % 120) as usize,
                    30 + ((rect_count * 91) % 80) as usize,
                    r_,
                    g_,
                    b_,
                );
                writeln!(output, "  rect done").ok();
            }
            "clear" => {
                let _ = output.reset(false);
                let _ = output.clear();
            }
            "cls" => {
                fb.clear(0, 0, 0);
                writeln!(output, "  fb cleared").ok();
            }
            "gui" => {
                fb.draw_text(50, 280, "Widgets Demo:", 255, 255, 255);
                let widgets = [
                    Widget::Button {
                        rect: gui::Rect { x: 50, y: 300, w: 120, h: 30 },
                        label: "Submit",
                        pressed: false,
                    },
                    Widget::Checkbox {
                        rect: gui::Rect { x: 200, y: 300, w: 120, h: 20 },
                        label: "Enable",
                        checked: true,
                    },
                    Widget::ProgressBar {
                        rect: gui::Rect { x: 50, y: 350, w: 300, h: 20 },
                        percent: 65,
                    },
                ];
                for w in &widgets {
                    w.draw(fb);
                }
                writeln!(output, "  gui demo drawn").ok();
            }
            "mouse" => {
                match mouse::init_mouse() {
                    Some(_) => writeln!(output, "  mouse stubbed").ok(),
                    None => writeln!(output, "  mouse init failed").ok(),
                }
            }
            "shot" => {
                match screenshot::save_screenshot(fb, "shot.bmp") {
                    Ok(_) => writeln!(output, "  saved shot.bmp").ok(),
                    Err(e) => writeln!(output, "  screenshot failed").ok(),
                }
            }
            "exit" => {
                writeln!(output, "  halting...").ok();
                loop {
                    boot::stall(Duration::from_secs(1));
                }
            }
            "" => {}
            _ => {
                writeln!(output, "  unknown command").ok();
            }
        }
    }
}

pub fn run(fb: &mut FrameBuffer) {
    let stdout_handle = boot::get_handle_for_protocol::<Output>().unwrap();
    let mut stdout = boot::open_protocol_exclusive::<Output>(stdout_handle).unwrap();
    let stdin_handle = boot::get_handle_for_protocol::<Input>().unwrap();
    let mut stdin = boot::open_protocol_exclusive::<Input>(stdin_handle).unwrap();
    shell_loop(&mut stdout, &mut stdin, fb);
}
