/*
 * encoding=utf-8
 * 散列操作接口。
 * 历史：
 *     2020-11-13，完成BASE16编解码操作。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
pub mod base16;
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
use std::slice;
/*============================================================================*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_CODE_BASE16_encode(
    raw: *const u8, len: usize, txt: *mut u8, cap: i8
) -> usize {
    let s = unsafe {
        slice::from_raw_parts(raw, len)
    };
    let d = unsafe {
        slice::from_raw_parts_mut(txt, len << 1)
    };

    base16::encode(s, d, cap != 0)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_CODE_BASE16_decode(
    txt: *const u8, len: usize, raw: *mut u8
) -> isize {
    let s = unsafe {
        slice::from_raw_parts(txt, len)
    };
    let d = unsafe {
        slice::from_raw_parts_mut(raw, len >> 1)
    };

    match base16::decode(s, d) {
        Ok(x) => {
            x as isize
        }
        Err(x) => {
            -(x as isize + 1_isize)
        }
    }
}
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
