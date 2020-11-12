use trueman_util_hash::bkdr::*;
/*============================================================================*/
fn main() {
    let key = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^".as_bytes();

    let x = x_0(key, 131, 0);
    println!("BKDR: {}", x);

    let x = time33(key, 0);
    println!("time33: {}", x);
}
