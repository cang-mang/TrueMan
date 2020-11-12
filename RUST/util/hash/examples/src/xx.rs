use trueman_util_hash::xx::*;
/*============================================================================*/
fn main() {
    let key = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^".as_bytes();

    let x = x32(key, 0x_FFFFFFFF_u32);
    println!("XX-32: {}", x);

    let x = x64(key, 0x_FFFFFFFF_FFFFFFFF_u64);
    println!("XX-64: {}", x);
}
