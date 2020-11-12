use trueman_crypto_hash::Ctx as _Ctx;
use trueman_crypto_hash::fnv128a::Ctx;
/*============================================================================*/
fn main() {
    let key = &[
        0x0D, 0x7D, 0x88, 0x74,   0x19, 0xCF, 0x27, 0x5A,
        0xFC, 0x51, 0x30, 0x33,   0x39, 0x01, 0x44, 0x5D,
        0xA8, 0xD3, 0xB9, 0x88,   0x78, 0x4C, 0x0A, 0xCF,
        0x08, 0x7D, 0x88, 0x74,   0x19, 0xCF, 0x27, 0x5A,
        0xFC, 0x01, 0xA3, 0x71,   0x5C, 0xBB, 0x79, 0xE0,
        0xF8, 0x5E, 0x2C, 0xA7,   0xEC, 0x45, 0x80, 0x01,
        0x52u8
    ];

    let mut ctx: Ctx = Default::default();
    let mut val: [u8; Ctx::LEN_DGT] = Default::default();

    ctx.update(key);
    ctx.r#final(&mut val);
    println!("hash for request: {:02X?}", val);

    for k in key {
        let slc = &[*k];
        ctx.update(slc);
    }

    ctx.r#final(&mut val);
    println!("hash for request: {:02X?}", val);

    Ctx::one(key, &mut val);
    println!("hash for request: {:02X?}", val);
}
