/*
 * encoding=utf-8
 * 散列操作接口。
 * 历史：
 *     2020-11-05，完成Jenkins-HASH散列操作。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
pub mod jenkins;
pub mod wang;
pub mod bkdr;
pub mod fnv1;
pub mod fnv1a;
pub mod xx;
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
use std::ffi::c_void;
use std::slice;
/*============================================================================*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_JENKINS_x_n(
    key: *const u32, cnt: usize, seed: u32
) -> u32 {
    let k = unsafe {
        slice::from_raw_parts(key, cnt)
    };
    jenkins::x_n(k, seed)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_JENKINS_x_3(
    a: u32, b: u32, c: u32, seed: u32
) -> u32 {
    jenkins::x_3(a, b, c, seed)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_JENKINS_x_2(
    a: u32, b: u32, seed: u32
) -> u32 {
    jenkins::x_2(a, b, seed)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_JENKINS_x_1(a: u32, seed: u32) -> u32 {
    jenkins::x_1(a, seed)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_JENKINS_x_0(
    key: *const c_void, len: usize, seed: u32
) -> u32 {
    let k = unsafe {
        slice::from_raw_parts(key as *const u8, len)
    };
    jenkins::x_0(k, seed)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_WANG_direct(key: u64) -> u64 {
    wang::direct(key)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_WANG_inverse(key: u64) -> u64 {
    wang::inverse(key)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_BKDR_x_0(
    key: *const c_void, len: usize, seed: u32, magic: u32
) -> u32 {
    let k = unsafe {
        slice::from_raw_parts(key as *const u8, len)
    };
    bkdr::x_0(k, seed, magic)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_BKDR_time33(
    key: *const c_void, len: usize, magic: u32
) -> u32 {
    let k = unsafe {
        slice::from_raw_parts(key as *const u8, len)
    };
    bkdr::time33(k, magic)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_FNV1_x32(
    key: *const c_void, len: usize
) -> u32 {
    let k = unsafe {
        slice::from_raw_parts(key as *const u8, len)
    };
    fnv1::x32(k)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_FNV1_x64(
    key: *const c_void, len: usize
) -> u64 {
    let k = unsafe {
        slice::from_raw_parts(key as *const u8, len)
    };
    fnv1::x64(k)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_FNV1A_x32(
    key: *const c_void, len: usize
) -> u32 {
    let k = unsafe {
        slice::from_raw_parts(key as *const u8, len)
    };
    fnv1a::x32(k)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_FNV1A_x64(
    key: *const c_void, len: usize
) -> u64 {
    let k = unsafe {
        slice::from_raw_parts(key as *const u8, len)
    };
    fnv1a::x64(k)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_XX_x32(
    key: *const c_void, len: usize, seed: u32
) -> u32 {
    let k = unsafe {
        slice::from_raw_parts(key as *const u8, len)
    };
    xx::x32(k, seed)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_HASH_XX_x64(
    key: *const c_void, len: usize, seed: u64
) -> u64 {
    let k = unsafe {
        slice::from_raw_parts(key as *const u8, len)
    };
    xx::x64(k, seed)
}
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
