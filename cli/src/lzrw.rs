use libc::{c_void, free, size_t};
use std::slice;

extern "C" {
    fn decompress(buffer: *mut u8, size: size_t, pSizeOut: *mut c_void) -> *mut u8;
    fn compress(buffer: *mut u8, size: size_t, pSizeOut: *mut c_void) -> *mut u8;
}

pub fn decompress_buffer(buffer: *const u8, size: u32) -> (&'static [u8], i32) {
    let mut size_out: i32 = 0;
    unsafe {
        let result = decompress(
            buffer as *mut u8,
            size as usize,
            &mut size_out as *mut i32 as *mut c_void,
        );

        let slice = slice::from_raw_parts(result, size_out as usize);
        free(result as *mut c_void);
        (slice, size_out)
    }
}

pub fn compress_buffer(buffer: *const u8, size: usize) -> Vec<u8> {
    let mut size_out: i32 = 0;
    unsafe {
        let result: *mut u8 = compress(
            buffer as *mut u8,
            size,
            &mut size_out as *mut i32 as *mut c_void,
        );

        Vec::from_raw_parts(result, size_out as usize, size_out as usize)
    }
}
