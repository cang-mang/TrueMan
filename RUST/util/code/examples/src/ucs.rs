use trueman_util_code::ucs2;
use trueman_util_code::ucs4;
/*============================================================================*/
fn my_tst(txt: &[u8]) {
    println!("raw {}", txt.len());
    for (pos, &cur) in txt.iter().enumerate() {
        println!("\t{}\t0x{:02X}", pos, cur);
    }

    let mut buf = [0_u16; 1024];
    let ret = ucs2::decode_utf8(txt, &mut buf);
    let len = match ret {
        Ok(x)  => {
            println!("complete UCS-2 {}", x);
            x
        }
        Err(x) => {
            println!("partial UCS-2 {}", x);
            x
        }
    };

    let slc = &buf[..len];
    for (pos, &cur) in slc.iter().enumerate() {
        println!("\t{}\t0x{:04X}", pos, cur);
    }

    let mut buf = [0_u8; 1024];
    let len = ucs2::encode_utf8(slc, &mut buf);
    println!("UTF-8 {}", len);
    let slc = &buf[..len];
    for (pos, &cur) in slc.iter().enumerate() {
        println!("\t{}\t0x{:02X}", pos, cur);
    }

    let mut buf = [0_u32; 1024];
    let ret = ucs4::decode_utf8(txt, &mut buf);
    let len = match ret {
        Ok(x)  => {
            println!("complete UCS-4 {}", x);
            x
        }
        Err(x) => {
            println!("partial UCS-4 {}", x);
            x
        }
    };

    let slc = &buf[..len];
    for (pos, &cur) in slc.iter().enumerate() {
        println!("\t{}\t0x{:08X}", pos, cur);
    }

    let mut buf = [0_u8; 1024];
    let len = ucs4::encode_utf8(slc, &mut buf);
    println!("UTF-8 {}", len);
    let slc = &buf[..len];
    for (pos, &cur) in slc.iter().enumerate() {
        println!("\t{}\t0x{:02X}", pos, cur);
    }
}
/*----------------------------------------------------------------------------*/
fn main() {
    let s = "𪚥𤉹";
    println!("Test \"{}\"", s);
    my_tst(s.as_bytes());

    let s = "汉字“𪚥”，不能在简体中文版Windows默认的代码页936（即GBK）中表示。";
    println!("Test \"{}\"", s);
    my_tst(s.as_bytes());

    let s = "โครงการแปลพระสูตรมหายานจีน-ไทยเฉลิมพระเกียรติ";
    println!("Test \"{}\"", s);
    my_tst(s.as_bytes());
}
