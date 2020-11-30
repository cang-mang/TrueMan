/*
 * encoding=utf-8
 * 网络报文校验和算法操作接口。
 * 历史：
 *     2020-11-12，完成。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
#[repr(C)]
pub struct Ctx {
    buf: [u8; 2],
    sum: u32
}
/*============================================================================*/
impl std::default::Default for Ctx {
    fn default() -> Self {
        Self {
            buf: [0_u8; 2],
            sum: 0_u32
        }
    }
}
/*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
use std::convert::TryFrom;
/*============================================================================*/
impl super::Ctx for Ctx {
    fn init(&mut self) -> &mut Self {
        self.buf[1] = 0_u8;
        self.sum    = 0_u32;
        self
    }
    /*------------------------------------------------------------------------*/
    fn update(&mut self, key: &[u8]) -> &mut Self {
        if key.len() != 0_usize {
            let mut pos = key.as_ptr();
            let mut len = key.len();

            if self.buf[1] != 0_u8 {
                self.buf[1] = unsafe {
                    *pos
                };

                self.sum = self.sum.wrapping_add(
                    u16::from_ne_bytes(self.buf) as u32
                );

                self.buf[1] = 0_u8;
                if len == 1_usize {
                    return self;
                }

                pos = unsafe {
                    pos.add(1_usize)
                };
                len -= 1_usize;
            }

            let odd = if pos as usize & 1_usize != 0_usize {
                true
            } else {
                false
            };
            if odd {
                if len == 1_usize {
                    self.buf[0] = unsafe {
                        *pos
                    };
                    self.buf[1] = 1_u8;
                    return self;
                }

                len -= 1_usize;
                self.buf[1] = unsafe {
                    *pos
                };

                pos = unsafe {
                    pos.add(1_usize)
                };
                len -= 1_usize;
            }

            let mut sum = 0_u32;
            if len >= 2_usize {
                if pos as usize & 2_usize != 0_usize {
                    let tmp = unsafe {
                        std::slice::from_raw_parts(pos as *const u16, 1_usize)
                    };

                    sum = tmp[0] as u32;

                    pos = unsafe {
                        pos.add(2_usize)
                    };
                    len -= 2_usize;
                }

                if len >= 4_usize {
                    let cnt = len >> 2;
                    let tmp = unsafe {
                        std::slice::from_raw_parts(pos as *const u32, cnt)
                    };

                    let mut cry = 0_u32;
                    for p in tmp {
                        let cur = *p;
                        sum = sum.wrapping_add(cry);
                        sum = sum.wrapping_add(cur);
                        cry = if cur > sum {
                            1_u32
                        } else {
                            0_u32
                        };
                    }

                    sum = sum.wrapping_add(cry);
                    sum = (sum & 0x_0000FFFF_u32) + (sum >> 16);

                    pos = unsafe {
                        pos.add(cnt << 2)
                    };
                }

                if len & 2_usize != 0_usize {
                    let tmp = unsafe {
                        std::slice::from_raw_parts(pos as *const u16, 1_usize)
                    };

                    sum = sum.wrapping_add(tmp[0] as u32);

                    pos = unsafe {
                        pos.add(2_usize)
                    };
                }
            }

            if len & 1_usize != 0_usize {
                self.buf[0] = unsafe {
                    *pos
                };

                if odd {
                    sum = sum.wrapping_add(u16::from_ne_bytes(self.buf) as u32);

                    self.buf[0] = unsafe {
                        *pos.add(1_usize)
                    };
                }

                self.buf[1] = 1_u8;
            } else if odd {
                self.buf[0] = unsafe {
                    *pos
                };

                sum = sum.wrapping_add(u16::from_ne_bytes(self.buf) as u32);
                self.buf[1] = 0_u8;
            }

            sum = (sum & 0x_0000FFFF_u32) + (sum >> 16);
            sum = (sum & 0x_0000FFFF_u32) + (sum >> 16);
            if odd {
                sum = ((sum >> 8) & 0x_000000FF_u32) |
                      ((sum & 0x_000000FF_u32) << 8);
            }

            self.sum = self.sum.wrapping_add(sum);
        }

        self
    }
    /*------------------------------------------------------------------------*/
    fn r#final(&mut self, val: &mut [u8]) -> &mut Self {
        if self.buf[1] != 0_u8 {
            self.buf[1] = 0_u8;
            self.sum = self.sum.wrapping_add(
                u16::from_ne_bytes(self.buf) as u32
            );
        }

        self.sum = (self.sum & 0x_0000FFFF_u32) + (self.sum >> 16);
        self.sum = (self.sum & 0x_0000FFFF_u32) + (self.sum >> 16);
        self.sum = !self.sum;

        let sum = self.sum as u16;
        let val = <&mut [u8; Self::LEN_DGT]>::try_from(
            &mut val[..Self::LEN_DGT]
        ).unwrap();

        *val = sum.to_ne_bytes();

        self.init()
    }
    /*------------------------------------------------------------------------*/
    fn one(key: &[u8], val: &mut [u8]) {
        let mut ctx: Self = Default::default();
        ctx.update(key).r#final(val);
    }
    /*------------------------------------------------------------------------*/
    const LEN_BLK: usize = 2_usize;
    const LEN_DGT: usize = 2_usize;
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
