/*
 * encoding=utf-8
 * 摘要操作接口。
 * 历史：
 *     2020-11-05，完成FNV128a-HASH算法操作。
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
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
pub mod fnv128a;
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
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
