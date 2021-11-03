extern crate libc;
extern crate lz4_sys;

pub mod liblz4;

mod decoder;
mod encoder;

pub mod block;

pub use crate::decoder::Decoder;
pub use crate::encoder::Encoder;
pub use crate::encoder::EncoderBuilder;
pub use crate::liblz4::version;
pub use crate::liblz4::BlockMode;
pub use crate::liblz4::BlockSize;
pub use crate::liblz4::ContentChecksum;

#[cfg(not(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
)))]
use libc::{c_char, size_t};

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
))]
use std::os::raw::c_char;

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
))]
#[allow(non_camel_case_types)]
type size_t = usize;

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
))]
#[export_name = "malloc"]
pub unsafe extern "C" fn malloc(size: usize) -> *mut usize {
    let new_size = size / std::mem::size_of::<usize>() + 2;
    let mut vec = std::mem::ManuallyDrop::new(Vec::with_capacity(new_size));
    vec.push(new_size);
    vec.as_mut_ptr().add(1)
}

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
))]
#[export_name = "calloc"]
pub unsafe extern "C" fn calloc(num: usize, size: usize) -> *mut usize {
    let new_size = (num * size) / std::mem::size_of::<usize>() + 2;
    let mut vec = std::mem::ManuallyDrop::new(Vec::with_capacity(new_size));
    vec.resize(new_size, 0);
    vec.push(new_size);
    vec.as_mut_ptr().add(1)
}

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
))]
#[export_name = "free"]
pub unsafe extern "C" fn free(ptr: *mut usize) {
    if ptr == std::ptr::null_mut() {
        return
    }
    let orig_ptr = ptr.sub(1);
    let vec = Vec::from_raw_parts(orig_ptr, *orig_ptr, *orig_ptr);
    drop(vec);
}

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
))]
#[export_name = "memset"]
pub unsafe extern "C" fn memset(dest: *mut u8, ch: u32, count: usize) -> *mut u8 {
    std::ptr::write_bytes(dest, ch as u8, count);
    dest
}

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
))]
#[export_name = "memcpy"]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *mut u8, num: usize) -> *mut u8 {
    std::ptr::copy_nonoverlapping(src, dest, num);
    dest
}

#[cfg(all(
    target_arch = "wasm32",
    not(any(target_env = "wasi", target_os = "wasi"))
))]
#[export_name = "memmove"]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *mut u8, num: usize) -> *mut u8 {
    std::ptr::copy(src, dest, num);
    dest
}
