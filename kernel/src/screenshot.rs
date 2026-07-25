extern crate alloc;
use alloc::alloc::{alloc, dealloc, Layout};

use crate::framebuffer::FrameBuffer;
use uefi::boot;
use uefi::proto::media::file::{File, FileAttribute, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;

pub fn save_screenshot(fb: &FrameBuffer, filename: &str) -> Result<(), &'static str> {
    let fs_handle = boot::get_handle_for_protocol::<SimpleFileSystem>()
        .map_err(|_| "no filesystem")?;
    let mut fs = boot::open_protocol_exclusive::<SimpleFileSystem>(fs_handle)
        .map_err(|_ | "open fs failed")?;
    let mut root = fs.open_volume().map_err(|_| "open volume failed")?;

    let path = uefi::CString16::try_from(filename).map_err(|_| "bad filename")?;
    let file = root.open(
        &*path,
        FileMode::CreateReadWrite,
        FileAttribute::empty(),
    ).map_err(|_| "open file failed")?;

    let mut regular = file.into_regular_file().ok_or("not regular file")?;

    let row_size = ((fb.width * 3 + 3) & !3) as usize;
    let bmp_size = 14 + 40 + row_size * fb.height;
    
    let layout = Layout::from_size_align(bmp_size, 1).map_err(|_| "layout failed")?;
    let buf = unsafe { alloc(layout) };
    if buf.is_null() {
        return Err("alloc failed");
    }

    unsafe {
        core::ptr::write_bytes(buf, 0, bmp_size);
    }

    unsafe {
        *buf.add(0) = b'B';
        *buf.add(1) = b'M';
        *buf.add(2) = (bmp_size & 0xFF) as u8;
        *buf.add(3) = ((bmp_size >> 8) & 0xFF) as u8;
        *buf.add(4) = ((bmp_size >> 16) & 0xFF) as u8;
        *buf.add(5) = ((bmp_size >> 24) & 0xFF) as u8;
        *buf.add(10) = 54;

        *buf.add(14) = 40;
        *buf.add(18) = (fb.width & 0xFF) as u8;
        *buf.add(19) = ((fb.width >> 8) & 0xFF) as u8;
        *buf.add(22) = (fb.height & 0xFF) as u8;
        *buf.add(23) = ((fb.height >> 8) & 0xFF) as u8;
        *buf.add(26) = 1;
        *buf.add(28) = 24;

        let src_stride = fb.stride * 4;
        for y in 0..fb.height {
            let src_offset = y * src_stride;
            let dst_offset = 54 + (fb.height - 1 - y) * row_size;
            for x in 0..fb.width {
                let src_idx = src_offset + x * 4;
                let dst_idx = dst_offset + x * 3;
                if src_idx + 3 < src_stride * fb.height {
                    *buf.add(dst_idx) = *fb.ptr.add(src_idx + 2);
                    *buf.add(dst_idx + 1) = *fb.ptr.add(src_idx + 1);
                    *buf.add(dst_idx + 2) = *fb.ptr.add(src_idx);
                }
            }
        }
    }

    let written = unsafe { core::slice::from_raw_parts(buf, bmp_size) };
    regular.write(written).map_err(|_| "write failed")?;

    unsafe { dealloc(buf, layout) };
    Ok(())
}
