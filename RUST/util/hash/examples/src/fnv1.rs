use trueman_util_hash::fnv1::*;
/*============================================================================*/
fn main() {
    let key = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^".as_bytes();

    let x = x32(key);
    println!("FNV1-32: {}", x);

    let x = x64(key);
    println!("FNV1-64: {}", x);
}
