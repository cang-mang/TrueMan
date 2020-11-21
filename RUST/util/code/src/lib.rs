/*
 * encoding=utf-8
 * 散列操作接口。
 * 历史：
 *     2020-11-13，完成BASE16编解码操作。
 *     2020-11-20，完成BASE64编解码操作。
 *     2020-11-20，完成UNICODE编解码操作。
 *     2020-11-21，完成无符号整数编解码操作。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
pub mod base16;
pub mod base64;
pub mod ucs2;
pub mod ucs4;
pub mod uintx;
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
use std::slice;
use self::uintx::Code;
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
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_CODE_BASE64_encode(
    raw: *const u8, len: usize, txt: *mut u8
) -> usize {
    let s = unsafe {
        slice::from_raw_parts(raw, len)
    };
    let d = unsafe {
        slice::from_raw_parts_mut(txt, ((len + 2) / 3) << 2)
    };

    base64::encode(s, d)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_CODE_BASE64_decode(
    txt: *const u8, len: usize, raw: *mut u8
) -> isize {
    let s = unsafe {
        slice::from_raw_parts(txt, len)
    };
    let d = unsafe {
        slice::from_raw_parts_mut(
            raw, ((len >> 2) << 1) + (len >> 2) + (len & 3)
        )
    };

    match base64::decode(s, d) {
        Ok(x) => {
            x as isize
        }
        Err(x) => {
            -(x as isize + 1_isize)
        }
    }
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_CODE_UCS2_encode_utf8(
    ucs: *const u16, len: usize, utf: *mut u8
) -> usize {
    let s = unsafe {
        slice::from_raw_parts(ucs, len)
    };
    let d = unsafe {
        slice::from_raw_parts_mut(utf, len * 3)
    };

    ucs2::encode_utf8(s, d)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_CODE_UCS2_decode_utf8(
    utf: *const u8, len: usize, ucs: *mut u16
) -> isize {
    let s = unsafe {
        slice::from_raw_parts(utf, len)
    };
    let d = unsafe {
        slice::from_raw_parts_mut(ucs, len)
    };

    match ucs2::decode_utf8(s, d) {
        Ok(x) => {
            x as isize
        }
        Err(x) => {
            -(x as isize + 1_isize)
        }
    }
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_CODE_UCS4_encode_utf8(
    ucs: *const u32, len: usize, utf: *mut u8
) -> usize {
    let s = unsafe {
        slice::from_raw_parts(ucs, len)
    };
    let d = unsafe {
        slice::from_raw_parts_mut(utf, len * 6)
    };

    ucs4::encode_utf8(s, d)
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_CODE_UCS4_decode_utf8(
    utf: *const u8, len: usize, ucs: *mut u32
) -> isize {
    let s = unsafe {
        slice::from_raw_parts(utf, len)
    };
    let d = unsafe {
        slice::from_raw_parts_mut(ucs, len)
    };

    match ucs4::decode_utf8(s, d) {
        Ok(x) => {
            x as isize
        }
        Err(x) => {
            -(x as isize + 1_isize)
        }
    }
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_CODE_UINTX_decode_be(
    pos: *const u8, len: usize
) -> u64 {
    let s = unsafe {
        slice::from_raw_parts(pos, len)
    };
    let mut ret: u64 = Default::default();

    ret.decode_be(s);
    ret
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_CODE_UINTX_decode_le(
    pos: *const u8, len: usize
) -> u64 {
    let s = unsafe {
        slice::from_raw_parts(pos, len)
    };
    let mut ret: u64 = Default::default();

    ret.decode_le(s);
    ret
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_CODE_UINTX_encode_be(
    pos: *mut u8, len: usize, val: u64
) {
    let d = unsafe {
        slice::from_raw_parts_mut(pos, len)
    };
    val.encode_be(d);
}
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub extern "C" fn TrueMan_UTIL_CODE_UINTX_encode_le(
    pos: *mut u8, len: usize, val: u64
) {
    let d = unsafe {
        slice::from_raw_parts_mut(pos, len)
    };
    val.encode_le(d);
}
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
