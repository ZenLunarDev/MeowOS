use crate::framebuffer::FrameBuffer;
use uefi::boot;
use uefi::proto::media::file::{File, FileAttribute, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;

pub fn save_screenshot(fb: &FrameBuffer, filename: &str) -> Result<(), &'static str> {
    let fs_handle = boot::get_handle_for_protocol::<SimpleFileSystem>()
        .map_err(|_| "no filesystem")?;
    let mut fs = boot::open_protocol_exclusive::<SimpleFileSystem>(fs_handle)
        .map_err(|_| "open fs failed")?;
    let mut root = fs.open_volume().map_err(|_| "open volume failed")?;

    let path = uefi::CString16::try_from(filename).map_err(|_| "bad filename")?;
    let mut file = root.open(
        &*path,
        FileMode::CreateReadWrite,
        FileAttribute::empty(),
    ).map_err(|_| "open file failed")?;

    let row_size = ((fb.width * 3 + 3) & !3) as usize;
    let bmp_size = 14 + 40 + row_size * fb.height;
    let buf_size = bmp_size;
    
    let layout = Layout::from_size_align(buf_size, 1).map_err(|_| "layout failed")?;
    let buf = core::alloc::alloc::alloc_zeroed(layout);
    if buf.is_null() {
        return Err("alloc failed");
    }

    buf[0] = b'B';
    buf[1] = b'M';
    buf[2] = (bmp_size & 0xFF) as u8;
    buf[3] = ((bmp_size >> 8) & 0xFF) as u8;
    buf[4] = ((bmp_size >> 16) & 0xFF) as u8;
    buf[5] = ((bmp_size >> 24) & 0xFF) as u8;
    buf[10] = 54;

    buf[14] = 40;
    buf[18] = (fb.width & 0xFF) as u8;
    buf[19] = ((fb.width >> 8) & 0xFF) as u8;
    buf[22] = (fb.height & 0xFF) as u8;
    buf[23] = ((fb.height >> 8) & 0xFF) as u8;
    buf[26] = 1;
    buf[28] = 24;

    let src_stride = fb.stride * 4;
    for y in 0..fb.height {
        let src_offset = y * src_stride;
        let dst_offset = 54 + (fb.height - 1 - y) * row_size;
        for x in 0..fb.width {
            let src_idx = src_offset + x * 4;
            let dst_idx = dst_offset + x * 3;
            if src_idx + 3 < src_stride * fb.height {
                buf[dst_idx] = unsafe { *fb.ptr.add(src_idx + 2) };
                buf[dst_idx + 1] = unsafe { *fb.ptr.add(src_idx + 1) };
                buf[dst_idx + 2] = unsafe { *fb.ptr.add(src_idx) };
            }
        }
    }

    let written = unsafe {
        core::slice::from_raw_parts(buf, buf_size)
    };
    file.write(written).map_err(|_| "write failed")?;

    core::alloc::alloc::dealloc(buf, layout);
    Ok(())
}
