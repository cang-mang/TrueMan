use trueman_util_code::base16::*;
/*============================================================================*/
fn my_tst(txt: &[u8], cap: bool) {
    println!("raw {}", txt.len());

    let mut buf0 = [255_u8; 128];
    let len0 = encode(txt, &mut buf0, cap);
    println!("encode {}", len0);
    let slc = &buf0[..len0];
    match std::str::from_utf8(slc) {
        Ok(s) => {
            println!("\t\"{}\"", s);
        }
        _     => {
            println!("\tFAILED");
        }
    }

    let mut buf1 = [255_u8; 128];
    let mut x = 0_usize;

    loop {
        let slc = &buf0[..(len0 - x)];
        let len1 = match decode(slc, &mut buf1) {
            Ok(y)  => {
                println!("complete BASE-16 {}", y);
                y
            }
            Err(y) => {
                println!("partial BASE-16 {}", y);
                y
            }
        };
        let slc = &buf1[..len1];
        match std::str::from_utf8(slc) {
            Ok(s) => {
                println!("\t\"{}\"", s);
            }
            _     => {
                println!("\tFAILED");
            }
        }

        x += 1;
        if x == 4 {
            break;
        }
    }
}
/*----------------------------------------------------------------------------*/
fn main() {
    let s = "#@";
    println!("Test \"{}\"", s);
    my_tst(s.as_bytes(), false);
    my_tst(s.as_bytes(), true);

    let s = "The quick brown fox jumps over the lazy dog.";
    println!("Test \"{}\"", s);
    my_tst(s.as_bytes(), false);
    my_tst(s.as_bytes(), true);
}
