/*
 * encoding=utf-8
 * 摘要操作接口。
 * 历史：
 *     2020-11-05，完成FNV128a-HASH算法操作。
 *     2020-11-12，完成网络报文校验和算法算法操作。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
use std::ffi::c_void;
/*============================================================================*/
#[repr(C)]
pub struct Op {
    init   : fn(*mut c_void                      ) -> *mut c_void,
    update : fn(*mut c_void, *const c_void, usize) -> *mut c_void,
    r#final: fn(*mut c_void, *mut u8             ) -> *mut c_void,
    /*------------------------------------------------------------------------*/
    one: fn(*const c_void, usize, *mut u8),
    /*------------------------------------------------------------------------*/
    len_ctx: usize,
    len_blk: usize,
    len_dgt: usize
}
/*============================================================================*/
pub trait Ctx {
    fn init   (&mut self                ) -> &mut Self;
    fn update (&mut self, key: &[u8]    ) -> &mut Self;
    fn r#final(&mut self, val: &mut [u8]) -> &mut Self;
    /*------------------------------------------------------------------------*/
    fn one(key: &[u8], val: &mut [u8]);
    /*------------------------------------------------------------------------*/
    const LEN_BLK: usize;
    const LEN_DGT: usize;
}
/*============================================================================*/
/*#[inline(always)]
fn zeroize(buf: &mut [u8]) {
    let mut cur = 0_usize;
    while cur < buf.len() {
        unsafe {
            std::ptr::write_volatile(buf.as_mut_ptr().add(cur), 0_u8);
        }

        cur += 1_usize;
    }
}*/
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
pub mod fnv128a;
pub mod csum;
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
#[no_mangle]
pub static TrueMan_CRYPTO_HASH_FNV128A_op: Op = Op {
    init   : fnv128a::init                      ,
    update : fnv128a::update                    ,
    r#final: fnv128a::r#final                   ,
    one    : fnv128a::one                       ,
    len_ctx: std::mem::size_of::<fnv128a::Ctx>(),
    len_blk: fnv128a::Ctx::LEN_BLK              ,
    len_dgt: fnv128a::Ctx::LEN_DGT
};
/*----------------------------------------------------------------------------*/
#[no_mangle]
pub static TrueMan_CRYPTO_HASH_CSUM_op: Op = Op {
    init   : csum::init                      ,
    update : csum::update                    ,
    r#final: csum::r#final                   ,
    one    : csum::one                       ,
    len_ctx: std::mem::size_of::<csum::Ctx>(),
    len_blk: csum::Ctx::LEN_BLK              ,
    len_dgt: csum::Ctx::LEN_DGT
};
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
