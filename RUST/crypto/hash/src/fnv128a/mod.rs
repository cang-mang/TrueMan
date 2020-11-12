/*
 * encoding=utf-8
 * FNV128a-HASH算法操作接口。
 * 历史：
 *     2020-11-05，完成。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
#[repr(C)]
pub struct Ctx {
    val: u128
}
/*============================================================================*/
impl std::default::Default for Ctx {
    fn default() -> Self {
        Self {
            val: 0x_6C62272E_07BB0142_62B82175_6295C58D_u128
        }
    }
}
/*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
impl super::Ctx for Ctx {
    fn init(&mut self) -> &mut Self {
        self.val = 0x_6C62272E_07BB0142_62B82175_6295C58D_u128;
        self
    }
    /*------------------------------------------------------------------------*/
    fn update(&mut self, key: &[u8]) -> &mut Self {
        for k in key {
            self.val ^= *k as u128;
            self.val = self.val.wrapping_mul(
                0x_00000000_01000000_00000000_0000013B_u128
            );
        }

        self
    }
    /*------------------------------------------------------------------------*/
    fn r#final(&mut self, val: &mut [u8]) -> &mut Self {
        self.val = self.val.to_le();

        assert!(val.len() >= Self::LEN_DGT);

        unsafe {
            std::ptr::copy_nonoverlapping(
                &self.val as *const u128 as *const u8,
                val.as_mut_ptr()                     ,
                Self::LEN_DGT
            );
        }

        self.init()
    }
    /*------------------------------------------------------------------------*/
    fn one(key: &[u8], val: &mut [u8]) {
        let mut ctx: Self = Default::default();
        ctx.update(key).r#final(val);
    }
    /*------------------------------------------------------------------------*/
    const LEN_BLK: usize =  1_usize;
    const LEN_DGT: usize = 12_usize;
}
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
use super::Ctx as _Ctx;
/*============================================================================*/
use std::ffi::c_void;
use std::slice;
/*============================================================================*/
pub(super) fn init(arg: *mut c_void) -> *mut c_void {
    let ctx: &mut Ctx = unsafe {
        &mut *(arg as *mut Ctx)
    };

    ctx.init();
    arg
}
/*----------------------------------------------------------------------------*/
pub(super) fn update(
    arg: *mut c_void, key: *const c_void, len: usize
) -> *mut c_void {
    let ctx: &mut Ctx = unsafe {
        &mut *(arg as *mut Ctx)
    };
    let k = unsafe {
        slice::from_raw_parts(key as *const u8, len)
    };

    ctx.update(k);
    arg
}
/*----------------------------------------------------------------------------*/
pub(super) fn r#final(arg: *mut c_void, val: *mut u8) -> *mut c_void {
    let ctx: &mut Ctx = unsafe {
        &mut *(arg as *mut Ctx)
    };
    let v = unsafe {
        slice::from_raw_parts_mut(val, Ctx::LEN_DGT)
    };

    ctx.r#final(v);
    arg
}
/*----------------------------------------------------------------------------*/
pub(super) fn one(key: *const c_void, len: usize, val: *mut u8) {
    let k = unsafe {
        slice::from_raw_parts(key as *const u8, len)
    };
    let v = unsafe {
        slice::from_raw_parts_mut(val, Ctx::LEN_DGT)
    };

    Ctx::one(k, v);
}
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
#[cfg(test)]
mod tests;
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
