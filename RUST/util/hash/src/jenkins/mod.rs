/*
 * encoding=utf-8
 * Jenkins-HASH散列操作接口。
 * 历史：
 *     2020-11-05，完成。
 * Jenkins-HASH算法很通用，能产生很好的分布，但耗时稍多。
 */
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
use std::convert::TryInto;
/*============================================================================*/
const MGC: u32 = 0x_DEAD_BEEF;
/*----------------------------------------------------------------------------*/
#[inline(always)]
fn fnl(a: &mut u32, b: &mut u32, c: &mut u32) {
    *c ^= *b; *c = c.wrapping_sub(b.rotate_left(14));
    *a ^= *c; *a = a.wrapping_sub(c.rotate_left(11));
    *b ^= *a; *b = b.wrapping_sub(a.rotate_left(25));
    *c ^= *b; *c = c.wrapping_sub(b.rotate_left(16));
    *a ^= *c; *a = a.wrapping_sub(c.rotate_left( 4));
    *b ^= *a; *b = b.wrapping_sub(a.rotate_left(14));
    *c ^= *b; *c = c.wrapping_sub(b.rotate_left(24));
}
/*----------------------------------------------------------------------------*/
#[inline(always)]
fn mix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *a = a.wrapping_sub(*c); *a ^= c.rotate_left( 4); *c = c.wrapping_add(*b);
    *b = b.wrapping_sub(*a); *b ^= a.rotate_left( 6); *a = a.wrapping_add(*c);
    *c = c.wrapping_sub(*b); *c ^= b.rotate_left( 8); *b = b.wrapping_add(*a);
    *a = a.wrapping_sub(*c); *a ^= c.rotate_left(16); *c = c.wrapping_add(*b);
    *b = b.wrapping_sub(*a); *b ^= a.rotate_left(19); *a = a.wrapping_add(*c);
    *c = c.wrapping_sub(*b); *c ^= b.rotate_left( 4); *b = b.wrapping_add(*a);
}
/*----------------------------------------------------------------------------*/
pub fn x_n(key: &[u32], seed: u32) -> u32 {
    let mut a = MGC.wrapping_add((key.len() as u32) << 2).wrapping_add(seed);
    let mut b = a;
    let mut c = b;
    let mut l = key.len();

    while l > 3 {
        let k = &key[(key.len() - l)..];
        l -= 3;
        a = a.wrapping_add(k[0]);
        b = b.wrapping_add(k[1]);
        c = c.wrapping_add(k[2]);
        mix(&mut a, &mut b, &mut c);
    }

    if l != 0 {
        let k = &key[(key.len() - l)..];
        loop {
            l -= 1;

            let x = k[l];
            match l {
                2 => {
                    c = c.wrapping_add(x);
                }
                1 => {
                    b = b.wrapping_add(x);
                }
                _ => {
                    a = a.wrapping_add(x);
                    fnl(&mut a, &mut b, &mut c);
                    break;
                }
            }
        }
    }

    c
}
/*----------------------------------------------------------------------------*/
fn x_4(a: u32, b: u32, c: u32, d: u32, seed: u32) -> u32 {
    let s = MGC.wrapping_add(d << 2).wrapping_add(seed);

    let mut x = a.wrapping_add(s);
    let mut y = b.wrapping_add(s);
    let mut z = c.wrapping_add(s);

    fnl(&mut x, &mut y, &mut z);
    z
}
/*----------------------------------------------------------------------------*/
#[inline(always)]
pub fn x_3(a: u32, b: u32, c: u32, seed: u32) -> u32 {
    x_4(a, b, c, 3_u32, seed)
}
/*----------------------------------------------------------------------------*/
#[inline(always)]
pub fn x_2(a: u32, b: u32, seed: u32) -> u32 {
    x_4(a, b, 0, 2_u32, seed)
}
/*----------------------------------------------------------------------------*/
#[inline(always)]
pub fn x_1(a: u32, seed: u32) -> u32 {
    x_4(a, 0, 0, 1_u32, seed)
}
/*----------------------------------------------------------------------------*/
pub fn x_0(key: &[u8], seed: u32) -> u32 {
    let mut a = MGC.wrapping_add(key.len() as u32).wrapping_add(seed);
    let mut b = a;
    let mut c = b;
    let mut k = key;

    while k.len() > 12 {
        let (x, v) = k.split_at(4);
        a = a.wrapping_add(u32::from_ne_bytes(x.try_into().unwrap()));

        let (x, w) = v.split_at(4);
        b = b.wrapping_add(u32::from_ne_bytes(x.try_into().unwrap()));

        let (x, v) = w.split_at(4);
        c = c.wrapping_add(u32::from_ne_bytes(x.try_into().unwrap()));

        mix(&mut a, &mut b, &mut c);
        k = v;
    }

    if k.len() != 0 {
        let mut l = k.len() - 1;
        loop {
            let x = k[l] as u32;
            match l {
                9..=11 => {
                    c = c.wrapping_add(x << ((l - 8) * 8));
                }
                8      => {
                    c = c.wrapping_add(x);
                }
                5..=7  => {
                    b = b.wrapping_add(x << ((l - 4) * 8));
                }
                4      => {
                    b = b.wrapping_add(x);
                }
                1..=3  => {
                    a = a.wrapping_add(x << ( l      * 8));
                }
                _      => {
                    a = a.wrapping_add(x);
                    fnl(&mut a, &mut b, &mut c);
                    break;
                }
            }

            l -= 1;
        }
    }

    c
}
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
#[cfg(test)]
mod tests;
/*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
