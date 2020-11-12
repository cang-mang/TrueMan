/*
 * encoding=utf-8
 * Wang-HASH散列操作接口。
 * 历史：
 *     2020-11-10，完成。
 * Wang-HASH是在Jenkins-HASH基础上进行的改进，能逆向计算。
 * Wang-HASH的逆运算会稍慢。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
pub fn direct(key: u64) -> u64 {
    let mut k = (!key).wrapping_add(key << 21);      k = k ^ (k >> 24);
    k = k.wrapping_add(k << 3).wrapping_add(k << 8); k = k ^ (k >> 14);
    k = k.wrapping_add(k << 2).wrapping_add(k << 4); k = k ^ (k >> 28);
    k.wrapping_add(k << 31)
}
/*----------------------------------------------------------------------------*/
pub fn inverse(key: u64) -> u64 {
    let mut x = key.wrapping_sub(key << 31);
    let mut y = key.wrapping_sub(x   << 31);

    x = y ^ (y >> 28); y = y ^ (x >> 28);
    y = y.wrapping_mul(14933078535860113213_u64);
    x = y ^ (y >> 14); x = y ^ (x >> 14);
    x = y ^ (x >> 14); y = y ^ (x >> 14);
    y = y.wrapping_mul(15244667743933553977_u64);
    x = y ^ (y >> 24); y = y ^ (x >> 24);
    x = !y;
    x = !(y.wrapping_sub(x << 21)); x = !(y.wrapping_sub(x << 21));
    !(y.wrapping_sub(x << 21))
}
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
#[cfg(test)]
mod tests;
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
