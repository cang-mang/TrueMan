/*
 * encoding=utf-8
 * MD5-HASH算法操作接口。
 * 历史：
 *     2020-11-30，完成。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
#[repr(C)]
pub struct Ctx {
    hsh: [u32;  4],
    blk: [u32; 16],
    cnt: u64
}
/*============================================================================*/
#[inline(always)]
fn f1(x: u32, y: u32, z: u32) -> u32 {
    z ^ (x & (y ^ z))
}
/*----------------------------------------------------------------------------*/
#[inline(always)]
fn f2(x: u32, y: u32, z: u32) -> u32 {
    f1(z, x, y)
}
/*----------------------------------------------------------------------------*/
#[inline(always)]
fn f3(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}
/*----------------------------------------------------------------------------*/
#[inline(always)]
fn f4(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | (!z))
}
/*----------------------------------------------------------------------------*/
#[inline(always)]
fn step(w: &mut u32, x: u32, s: u32) {
    *w = w.rotate_left(s).wrapping_add(x);
}
/*----------------------------------------------------------------------------*/
#[inline(always)]
fn step1(w: &mut u32, x: u32, y: u32, z: u32, i: u32, j: u32, s: u32) {
    *w = w.wrapping_add(f1(x, y, z)).wrapping_add(i).wrapping_add(j);
    step(w, x, s);
}
/*----------------------------------------------------------------------------*/
#[inline(always)]
fn step2(w: &mut u32, x: u32, y: u32, z: u32, i: u32, j: u32, s: u32) {
    *w = w.wrapping_add(f2(x, y, z)).wrapping_add(i).wrapping_add(j);
    step(w, x, s);
}
/*----------------------------------------------------------------------------*/
#[inline(always)]
fn step3(w: &mut u32, x: u32, y: u32, z: u32, i: u32, j: u32, s: u32) {
    *w = w.wrapping_add(f3(x, y, z)).wrapping_add(i).wrapping_add(j);
    step(w, x, s);
}
/*----------------------------------------------------------------------------*/
#[inline(always)]
fn step4(w: &mut u32, x: u32, y: u32, z: u32, i: u32, j: u32, s: u32) {
    *w = w.wrapping_add(f4(x, y, z)).wrapping_add(i).wrapping_add(j);
    step(w, x, s);
}
/*----------------------------------------------------------------------------*/
fn transform(hsh: &mut [u32; 4], i: &[u32; 16]) {
    let mut a = hsh[0];
    let mut b = hsh[1];
    let mut c = hsh[2];
    let mut d = hsh[3];

    step1(&mut a, b, c, d, i[ 0], 0x_D76AA478_u32,  7_u32);
    step1(&mut d, a, b, c, i[ 1], 0x_E8C7B756_u32, 12_u32);
    step1(&mut c, d, a, b, i[ 2], 0x_242070DB_u32, 17_u32);
    step1(&mut b, c, d, a, i[ 3], 0x_C1BDCEEE_u32, 22_u32);
    step1(&mut a, b, c, d, i[ 4], 0x_F57C0FAF_u32,  7_u32);
    step1(&mut d, a, b, c, i[ 5], 0x_4787C62A_u32, 12_u32);
    step1(&mut c, d, a, b, i[ 6], 0x_A8304613_u32, 17_u32);
    step1(&mut b, c, d, a, i[ 7], 0x_FD469501_u32, 22_u32);
    step1(&mut a, b, c, d, i[ 8], 0x_698098D8_u32,  7_u32);
    step1(&mut d, a, b, c, i[ 9], 0x_8B44F7AF_u32, 12_u32);
    step1(&mut c, d, a, b, i[10], 0x_FFFF5BB1_u32, 17_u32);
    step1(&mut b, c, d, a, i[11], 0x_895CD7BE_u32, 22_u32);
    step1(&mut a, b, c, d, i[12], 0x_6B901122_u32,  7_u32);
    step1(&mut d, a, b, c, i[13], 0x_FD987193_u32, 12_u32);
    step1(&mut c, d, a, b, i[14], 0x_A679438E_u32, 17_u32);
    step1(&mut b, c, d, a, i[15], 0x_49B40821_u32, 22_u32);

    step2(&mut a, b, c, d, i[ 1], 0x_F61E2562_u32,  5_u32);
    step2(&mut d, a, b, c, i[ 6], 0x_C040B340_u32,  9_u32);
    step2(&mut c, d, a, b, i[11], 0x_265E5A51_u32, 14_u32);
    step2(&mut b, c, d, a, i[ 0], 0x_E9B6C7AA_u32, 20_u32);
    step2(&mut a, b, c, d, i[ 5], 0x_D62F105D_u32,  5_u32);
    step2(&mut d, a, b, c, i[10], 0x_02441453_u32,  9_u32);
    step2(&mut c, d, a, b, i[15], 0x_D8A1E681_u32, 14_u32);
    step2(&mut b, c, d, a, i[ 4], 0x_E7D3FBC8_u32, 20_u32);
    step2(&mut a, b, c, d, i[ 9], 0x_21E1CDE6_u32,  5_u32);
    step2(&mut d, a, b, c, i[14], 0x_C33707D6_u32,  9_u32);
    step2(&mut c, d, a, b, i[ 3], 0x_F4D50D87_u32, 14_u32);
    step2(&mut b, c, d, a, i[ 8], 0x_455A14ED_u32, 20_u32);
    step2(&mut a, b, c, d, i[13], 0x_A9E3E905_u32,  5_u32);
    step2(&mut d, a, b, c, i[ 2], 0x_FCEFA3F8_u32,  9_u32);
    step2(&mut c, d, a, b, i[ 7], 0x_676F02D9_u32, 14_u32);
    step2(&mut b, c, d, a, i[12], 0x_8D2A4C8A_u32, 20_u32);

    step3(&mut a, b, c, d, i[ 5], 0x_FFFA3942_u32,  4_u32);
    step3(&mut d, a, b, c, i[ 8], 0x_8771F681_u32, 11_u32);
    step3(&mut c, d, a, b, i[11], 0x_6D9D6122_u32, 16_u32);
    step3(&mut b, c, d, a, i[14], 0x_FDE5380C_u32, 23_u32);
    step3(&mut a, b, c, d, i[ 1], 0x_A4BEEA44_u32,  4_u32);
    step3(&mut d, a, b, c, i[ 4], 0x_4BDECFA9_u32, 11_u32);
    step3(&mut c, d, a, b, i[ 7], 0x_F6BB4B60_u32, 16_u32);
    step3(&mut b, c, d, a, i[10], 0x_BEBFBC70_u32, 23_u32);
    step3(&mut a, b, c, d, i[13], 0x_289B7EC6_u32,  4_u32);
    step3(&mut d, a, b, c, i[ 0], 0x_EAA127FA_u32, 11_u32);
    step3(&mut c, d, a, b, i[ 3], 0x_D4EF3085_u32, 16_u32);
    step3(&mut b, c, d, a, i[ 6], 0x_04881D05_u32, 23_u32);
    step3(&mut a, b, c, d, i[ 9], 0x_D9D4D039_u32,  4_u32);
    step3(&mut d, a, b, c, i[12], 0x_E6DB99E5_u32, 11_u32);
    step3(&mut c, d, a, b, i[15], 0x_1FA27CF8_u32, 16_u32);
    step3(&mut b, c, d, a, i[ 2], 0x_C4AC5665_u32, 23_u32);

    step4(&mut a, b, c, d, i[ 0], 0x_F4292244_u32,  6_u32);
    step4(&mut d, a, b, c, i[ 7], 0x_432AFF97_u32, 10_u32);
    step4(&mut c, d, a, b, i[14], 0x_AB9423A7_u32, 15_u32);
    step4(&mut b, c, d, a, i[ 5], 0x_FC93A039_u32, 21_u32);
    step4(&mut a, b, c, d, i[12], 0x_655B59C3_u32,  6_u32);
    step4(&mut d, a, b, c, i[ 3], 0x_8F0CCC92_u32, 10_u32);
    step4(&mut c, d, a, b, i[10], 0x_FFEFF47D_u32, 15_u32);
    step4(&mut b, c, d, a, i[ 1], 0x_85845DD1_u32, 21_u32);
    step4(&mut a, b, c, d, i[ 8], 0x_6FA87E4F_u32,  6_u32);
    step4(&mut d, a, b, c, i[15], 0x_FE2CE6E0_u32, 10_u32);
    step4(&mut c, d, a, b, i[ 6], 0x_A3014314_u32, 15_u32);
    step4(&mut b, c, d, a, i[13], 0x_4E0811A1_u32, 21_u32);
    step4(&mut a, b, c, d, i[ 4], 0x_F7537E82_u32,  6_u32);
    step4(&mut d, a, b, c, i[11], 0x_BD3AF235_u32, 10_u32);
    step4(&mut c, d, a, b, i[ 2], 0x_2AD7D2BB_u32, 15_u32);
    step4(&mut b, c, d, a, i[ 9], 0x_EB86D391_u32, 21_u32);

    hsh[0] = hsh[0].wrapping_add(a);
    hsh[1] = hsh[1].wrapping_add(b);
    hsh[2] = hsh[2].wrapping_add(c);
    hsh[3] = hsh[3].wrapping_add(d);
}
/*----------------------------------------------------------------------------*/
impl Ctx {
    #[inline(always)]
    fn mix(&mut self) {
        super::u32_le2cpu_v(&mut self.blk);
        transform(&mut self.hsh, &self.blk);
    }
}
/*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
impl std::default::Default for Ctx {
    fn default() -> Self {
        Self {
            hsh: [
                0x_67452301_u32,
                0x_EFCDAB89_u32,
                0x_98BADCFE_u32,
                0x_10325476_u32
            ],
            blk: [0_u32; 16],
            cnt: 0_u64
        }
    }
}
/*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
impl super::Ctx for Ctx {
    fn init(&mut self) -> &mut Self {
        self.hsh[0] = 0x_67452301_u32;
        self.hsh[1] = 0x_EFCDAB89_u32;
        self.hsh[2] = 0x_98BADCFE_u32;
        self.hsh[3] = 0x_10325476_u32;
        self.cnt    = 0_u64;
        self
    }
    /*------------------------------------------------------------------------*/
    fn update(&mut self, key: &[u8]) -> &mut Self {
        let lft = Self::LEN_BLK - (self.cnt & 0x_3F_u64) as usize;
        self.cnt += key.len() as u64;

        let blk = self.blk.as_mut_ptr() as *mut u8;
        if lft > key.len() {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    key.as_ptr(), blk.add(Self::LEN_BLK - lft), key.len()
                );
            }
        } else {
            let mut pos = key.as_ptr();
            let mut len = key.len();

            unsafe {
                std::ptr::copy_nonoverlapping(
                    pos, blk.add(Self::LEN_BLK - lft), lft
                );
            }

            self.mix();

            pos = unsafe {
                pos.add(lft)
            };

            len -= lft;
            while len >= Self::LEN_BLK {
                unsafe {
                    std::ptr::copy_nonoverlapping(pos, blk, Self::LEN_BLK);
                }

                self.mix();

                pos = unsafe {
                    pos.add(Self::LEN_BLK)
                };

                len -= Self::LEN_BLK;
            }

            unsafe {
                std::ptr::copy_nonoverlapping(pos, blk, len);
            }
        }

        self
    }
    /*------------------------------------------------------------------------*/
    fn r#final(&mut self, val: &mut [u8]) -> &mut Self {
        let     offset  = (self.cnt & 0x_3F_u64) as usize;
        let mut padding = 55_isize - offset as isize;

        let     blk = self.blk.as_mut_ptr() as *mut u8;
        let mut cur = unsafe {
            blk.add(offset)
        };

        cur = unsafe {
            *cur = 0x_80_u8;
            cur.add(1_usize)
        };

        if padding < 0_isize {
            unsafe {
                std::ptr::write_bytes(cur, 0_u8, (padding + 8_isize) as usize);
            }

            self.mix();
            cur = blk;
            padding = 56_isize;
        }

        unsafe {
            std::ptr::write_bytes(cur, 0_u8, padding as usize);
        }

        self.blk[14] = (self.cnt <<  3) as u32;
        self.blk[15] = (self.cnt >> 29) as u32;
        super::u32_le2cpu_v(&mut self.blk[..14]);
        transform(&mut self.hsh, &self.blk);
        super::u32_cpu2le_v(&mut self.hsh);

        assert!(val.len() >= Self::LEN_DGT);

        unsafe {
            std::ptr::copy_nonoverlapping(
                self.hsh.as_ptr() as *const u8, val.as_mut_ptr(), Self::LEN_DGT
            );
        }

        super::zeroize(
            unsafe {
                std::slice::from_raw_parts_mut(
                    self.blk.as_mut_ptr() as *mut u8, Self::LEN_BLK
                )
            }
        );
        self.init()
    }
    /*------------------------------------------------------------------------*/
    fn one(key: &[u8], val: &mut [u8]) {
        let mut ctx: Self = Default::default();
        ctx.update(key).r#final(val);
    }
    /*------------------------------------------------------------------------*/
    const LEN_BLK: usize = 64_usize;
    const LEN_DGT: usize = 16_usize;
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
