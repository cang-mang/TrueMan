use trueman_util_hash::fnv1a::*;
/*============================================================================*/
fn main() {
    let key = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^".as_bytes();

    let x = x32(key);
    println!("FNV1a-32: {}", x);

    let x = x64(key);
    println!("FNV1a-64: {}", x);
}
