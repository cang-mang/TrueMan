use trueman_util_hash::wang::*;
/*============================================================================*/
fn main() {
    let x = 9876543210123456789_u64;
    let y = direct(x);
    println!("wang direct: {}=->{}", x, y);

    let x = inverse(y);
    println!("wang inverse: {}=->{}", y, x);
}
