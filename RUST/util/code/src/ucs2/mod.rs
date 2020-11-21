/*
 * encoding=utf-8
 * UCS-2编解码操作接口。
 * 历史：
 *     2020-11-20，完成。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
//将UCS-2字符串转成UTF-8编码。
//每个UCS-2字符的取值范围是[0x0000, 0xFFFF]，即16位无符号整数的完整范围。
//1个UCS-2字符最多转成3个字节的UTF-8编码。
//返回实际写入utf的字节数。
pub fn encode_utf8(ucs: &[u16], utf: &mut [u8]) -> usize {
    let mut lft = 0_u8;
    let mut cnt = 0_usize;

    for x in ucs {
        let cur = *x;
        if cur < 0x_00_80_u16 {
            utf[cnt] =                  cur         as u8;
        } else if cur < 0x_08_00_u16 {
            lft = 1;
            utf[cnt] = (0x_00_C0_u16 | (cur >>  6)) as u8;
        } else {
            lft = 2;
            utf[cnt] = (0x_00_E0_u16 | (cur >> 12)) as u8;
        }

        cnt += 1;
        if lft != 0 {
            loop {
                lft -= 1;
                if lft == 0 {
                    utf[cnt] = (0x_00_80_u16 | (0x_00_3F_u16 & cur)) as u8;
                    cnt += 1;
                    break;
                }

                utf[cnt] = (0x_00_80_u16 | (0x_00_3F_u16 & (cur >> 6))) as u8;
                cnt += 1;
            }
        }
    }

    cnt
}
/*----------------------------------------------------------------------------*/
//从UTF-8解码得到UCS-2字符串。
//最少1个字节、最多3个字节的UTF-8编码转成1个UCS-2字符。
//无论返回成功还是失败，都会携带写入ucs多少个元素。
pub fn decode_utf8(utf: &[u8], ucs: &mut [u16]) -> Result<usize, usize> {
    let mut u   = 0_u16;
    let mut lft = 0_i8;
    let mut cnt = 0_usize;

    for x in utf {
        let cur = *x;
        if lft != 0 {
            if cur & 0x_C0_u8 == 0x_80_u8 {
                u <<= 6;
                u |= (cur & 0x_3F_u8) as u16;
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
                u = (cur & 0x_1F_u8) as u16;
                lft = 1;
            } else if cur & 0x_F0_u8 == 0x_E0_u8 {
                u = (cur & 0x_0F_u8) as u16;
                lft = 2;
            } else {
                lft = -1;
                break;
            }
        } else {
            ucs[cnt] = cur as u16;
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
