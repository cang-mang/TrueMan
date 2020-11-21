/*
 * encoding=utf-8
 * UCS-4编解码操作接口。
 * 历史：
 *     2020-11-20，完成。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
//将UCS-4字符串转成UTF-8编码。
//每个UCS-4字符的取值范围是[0x00000000, 0x7FFFFFFF]，
//即32位有符号整数的非负范围。
//1个UCS-4字符最多转成6个字节的UTF-8编码。
//返回实际写入utf的字节数。
pub fn encode_utf8(ucs: &[u32], utf: &mut [u8]) -> usize {
    let mut lft = 0_u8;
    let mut cnt = 0_usize;

    for x in ucs {
        let cur = *x;
        if cur < 0x_00_00_00_80_u32 {
            utf[cnt] =                        cur         as u8;
        } else if cur < 0x_00_00_08_00_u32 {
            lft = 1;
            utf[cnt] = (0x_00_00_00_C0_u32 | (cur >>  6)) as u8;
        } else if cur < 0x_00_01_00_00_u32 {
            lft = 2;
            utf[cnt] = (0x_00_00_00_E0_u32 | (cur >> 12)) as u8;
        } else if cur < 0x_00_20_00_00_u32 {
            lft = 3;
            utf[cnt] = (0x_00_00_00_F0_u32 | (cur >> 18)) as u8;
        } else if cur < 0x_04_00_00_00_u32 {
            lft = 4;
            utf[cnt] = (0x_00_00_00_F8_u32 | (cur >> 24)) as u8;
        } else if cur < 0x_80_00_00_00_u32 {
            lft = 5;
            utf[cnt] = (0x_00_00_00_FC_u32 | (cur >> 30)) as u8;
        } else {
            break;
        }

        cnt += 1;
        if lft != 0 {
            loop {
                lft -= 1;
                match lft {
                    0 => {
                        utf[cnt] = (
                            0x_00_00_00_80_u32 | (0x_00_00_00_3F_u32 & cur)
                        ) as u8;
                        cnt += 1;
                        break;
                    }
                    _ => {
                        utf[cnt] = (
                            0x_00_00_00_80_u32 | (
                                0x_00_00_00_3F_u32 & (cur >> (6 * lft))
                            )
                        ) as u8;
                        cnt += 1;
                    }
                }
            }
        }
    }

    cnt
}
/*----------------------------------------------------------------------------*/
//从UTF-8解码得到UCS-4字符串。
//最少1个字节、最多6个字节的UTF-8编码转成1个UCS-4字符。
//无论返回成功还是失败，都会携带写入ucs多少个元素。
pub fn decode_utf8(utf: &[u8], ucs: &mut [u32]) -> Result<usize, usize> {
    let mut u   = 0_u32;
    let mut lft = 0_i8;
    let mut cnt = 0_usize;

    for x in utf {
        let cur = *x;
        if lft != 0 {
            if cur & 0x_C0_u8 == 0x_80_u8 {
                u <<= 6;
                u |= (cur & 0x_3F_u8) as u32;
                lft -= 1;
                if lft == 0 {
                    ucs[cnt] = u;
                    cnt += 1;
                }
            } else {
                lft = -1;
                break;
            }
        } else if cur & 0x_80_u8 != 0 {
            if cur & 0x_E0_u8 == 0x_C0_u8 {
                u = (cur & 0x_1F_u8) as u32;
                lft = 1;
            } else if cur & 0x_F0_u8 == 0x_E0_u8 {
                u = (cur & 0x_0F_u8) as u32;
                lft = 2;
            } else if cur & 0x_F8_u8 == 0x_F0_u8 {
                u = (cur & 0x_07_u8) as u32;
                lft = 3;
            } else if cur & 0x_FC_u8 == 0x_F8_u8 {
                u = (cur & 0x_03_u8) as u32;
                lft = 4;
            } else if cur & 0x_FE_u8 == 0x_FC_u8 {
                u = (cur & 0x_01_u8) as u32;
                lft = 5;
            } else {
                lft = -1;
                break;
            }
        } else {
            ucs[cnt] = cur as u32;
            cnt += 1;
        }
    }

    if lft == 0 {
        Ok(cnt)
    } else {
        Err(cnt)
    }
}
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
#[cfg(test)]
mod tests;
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
