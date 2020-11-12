use trueman_util_hash::jenkins::*;
/*============================================================================*/
fn main() {
    let seed = 0xe3630b12_u32;

    let key = &[
        0x5b_u8, 0x26_u8, 0xb9_u8, 0xf6_u8,
        0xb9_u8, 0x16_u8, 0x02_u8, 0x22_u8,
        0x52_u8, 0xf2_u8, 0xdf_u8, 0x4f_u8
    ];

    let x = x_0(key, seed);
    println!("jenkins bin: {}", x);

    let key = &[0xf6b9265b_u32, 0x220216b9_u32, 0x4fdff252_u32];

    let x = x_n(key, seed);
    println!("jenkins u32: {}", x);

    let x = x_3(key[0], key[1], key[2], seed);
    println!("jenkins 3: {}", x);

    let x = x_2(key[0], key[1], seed);
    println!("jenkins 2: {}", x);

    let x = x_1(key[0], seed);
    println!("jenkins 1: {}", x);
}
