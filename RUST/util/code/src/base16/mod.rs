/*
 * encoding=utf-8
 * BASE16编解码操作接口。
 * 历史：
 *     2020-11-13，完成。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
const LOWER: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
    b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f'
];
/*----------------------------------------------------------------------------*/
const UPPER: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
    b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F'
];
/*----------------------------------------------------------------------------*/
//1字节的数据，编码后得到2字节。
//txt的字节数至少是len乘以二，所以txt的字节数至少为：
//    len * 2
//用位操作加速可写为：
//    len << 1
//cap表示字母是否用大写形式输出。
//返回实际写入txt的字节数。
pub fn encode(raw: &[u8], txt: &mut [u8], cap: bool) -> usize {
    let tab = if cap {
        &UPPER
    } else {
        &LOWER
    };

    let mut cnt = 0_usize;
    for x in raw {
        let cur = *x;
        txt[cnt] = tab[(cur >> 4) as usize]; cnt += 1;
        txt[cnt] = tab[(cur & 15) as usize]; cnt += 1;
    }

    cnt
}
/*----------------------------------------------------------------------------*/
//2字节的数据，解码后得到1字节。
//len应是2的整数倍；raw的字节数至少为：
//    len / 2
//用位操作加速可写为：
//    len >> 1
//无论返回成功还是失败，都会携带写入raw多少字节。
pub fn decode(txt: &[u8], raw: &mut [u8]) -> Result<usize, usize> {
    let mut tmp = 0_u8;
    let mut odd = true;
    let mut cnt = 0_usize;

    for x in txt {
        let mut cur = *x;
        match cur {
            b'0'..=b'9' => {
                cur -= b'0';
            }
            b'a'..=b'f' => {
                cur -= b'a' - 10;
            }
            b'A'..=b'F' => {
                cur -= b'A' - 10;
            }
            _           => {
                return Err(cnt);
            }
        }

        if odd {
            odd = false;
            tmp = cur;
        } else {
            odd = true;
            tmp <<= 4;
            tmp |= cur;
            raw[cnt] = tmp;
            cnt += 1;
        }
    }

    if txt.len() & 1 == 0 {
        Ok(cnt)
    } else {
        Err(cnt)
    }
}
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
#[cfg(test)]
mod tests;
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
