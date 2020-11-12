/*
 * encoding=utf-8
 * BKDR-HASH散列操作接口。
 * 历史：
 *     2020-11-10，完成。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
//适用于对字符串进行计算。
//seed可以取31、131、1313、13131、131313等。
//magic是HASH初始值，一般取0即可。
//如果字符串内容主要是英文字母和数字字符，seed建议取31；一般seed是取131。
pub fn x_0(key: &[u8], seed: u32, magic: u32) -> u32 {
    let mut h = magic;
    for k in key {
        h = h.wrapping_mul(seed).wrapping_add(*k as u32);
    }

    h
}
/*----------------------------------------------------------------------------*/
//适用于对短小的由英文字母组成的字符串进行计算。
//magic是HASH初始值，一般取0即可。
#[inline(always)]
pub fn time33(key: &[u8], magic: u32) -> u32 {
    x_0(key, 33_u32, magic)
}
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
#[cfg(test)]
mod tests;
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
