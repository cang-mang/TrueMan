/*
 * encoding=utf-8
 * BASE64编解码操作接口。
 * 历史：
 *     2020-11-20，完成。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
const TAB: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G',
    b'H', b'I', b'J', b'K', b'L', b'M', b'N',
    b'O', b'P', b'Q',       b'R', b'S', b'T',
    b'U', b'V', b'W',       b'X', b'Y', b'Z',
    b'a', b'b', b'c', b'd', b'e', b'f', b'g',
    b'h', b'i', b'j', b'k', b'l', b'm', b'n',
    b'o', b'p', b'q',       b'r', b's', b't',
    b'u', b'v', b'w',       b'x', b'y', b'z',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'+', b'/'
];
/*----------------------------------------------------------------------------*/
//3字节的数据，编码后得到4字节。
//txt的字节数至少是len乘以三分之四。
//由于len不一定被3整除，所以txt的字节数至少为：
//    ((len + 2) / 3) * 4
//用位操作加速可写为：
//    ((len + 2) / 3) << 2
//返回实际写入txt的字节数。
pub fn encode(raw: &[u8], txt: &mut [u8]) -> usize {
    let mut nxt = 0_u8;
    let mut cnt = 0_usize;

    for (pos, x) in raw.iter().enumerate() {
        let mut cur = *x;
        match pos % 3 {
            2 => {
                nxt  |=  cur >> 6            ; txt[cnt]  = TAB[nxt as usize];
                                               cnt      += 1                ;
                cur  &=  0x_3F_u8            ; txt[cnt]  = TAB[cur as usize];
                                               cnt      += 1                ;
            }
            1 => {
                nxt  |=  cur >> 4            ; txt[cnt]  = TAB[nxt as usize];
                                               cnt      += 1                ;
                nxt   = (cur << 2) & 0x_3F_u8;
            }
            _ => {
                nxt   = (cur << 4) & 0x_3F_u8;
                cur >>=  2                   ; txt[cnt]  = TAB[cur as usize];
                                               cnt      += 1                ;
            }
        }
    }

    let len = raw.len() % 3;
    if len != 0 {
        txt[cnt] = TAB[nxt as usize]; cnt += 1;
        txt[cnt] = b'='             ; cnt += 1;
        if len == 1 {
            txt[cnt] = b'='         ; cnt += 1;
        }
    }

    cnt
}
/*----------------------------------------------------------------------------*/
//4字节的数据，解码后得到3字节。
//len应是4的整数倍；raw的字节数至少为：
//    ((len / 4) * 3) + (len % 4)
//用位操作加速可写为：
//    ((len >> 2) << 1) + (len >> 2) + (len & 3)
//而实际写入raw的字节数可能更少，尤其当len不是4的整数倍时。
//无论返回成功还是失败，都会携带写入raw多少字节。
pub fn decode(txt: &[u8], raw: &mut [u8]) -> Result<usize, usize> {
    let mut tmp = 0_u8;
    let mut cnt = 0_usize;

    for (pos, x) in txt.iter().enumerate() {
        let mut cur = *x;
        match cur {
            b'A'..=b'Z' => {
                cur -= b'A';
            }
            b'a'..=b'z' => {
                cur -= b'a' - 26;
            }
            b'0'..=b'9' => {
                cur += 52 - b'0';
            }
            b'+'        => {
                cur = 62;
            }
            b'/'        => {
                cur = 63;
            }
            b'='        => {
                match pos & 3 {
                    3 => {
                        if (tmp != 0) || (pos + 1 != txt.len()) {
                            return Err(cnt);
                        }
                    }
                    2 => {
                        if (tmp != 0) || (pos + 2 != txt.len())
                                      || (txt[pos + 1] != b'=')
                        {
                            return Err(cnt);
                        }
                    }
                    _ => {
                        return Err(cnt);
                    }
                }

                return Ok(cnt);
            }
            _           => {
                return Err(cnt);
            }
        }

        match pos & 3 {
            3 => {
                tmp |= cur     ; raw[cnt] = tmp; cnt += 1;
            }
            2 => {
                tmp |= cur >> 2; raw[cnt] = tmp; cnt += 1;
                tmp  = cur << 6;
            }
            1 => {
                tmp |= cur >> 4; raw[cnt] = tmp; cnt += 1;
                tmp  = cur << 4;
            }
            _ => {
                tmp  = cur << 2;
            }
        }
    }

    if txt.len() & 3 == 0 {
        Ok(cnt)
    } else {
        Err(cnt)
    }
}
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
#[cfg(test)]
mod tests;
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
