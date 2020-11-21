#[test]
fn exe() {
    let src = "𪚥𤉹".as_bytes();
    assert_eq!(src.len(), 8);
    assert_eq!(src[0], 0xF0);
    assert_eq!(src[1], 0xAA);
    assert_eq!(src[2], 0x9A);
    assert_eq!(src[3], 0xA5);
    assert_eq!(src[4], 0xF0);
    assert_eq!(src[5], 0xA4);
    assert_eq!(src[6], 0x89);
    assert_eq!(src[7], 0xB9);

    let mut d_0 = [1234567890_u32; 128];
    let l_0 = match super::decode_utf8(src, &mut d_0) {
        Ok(y)  => {
            y
        }
        Err(y) => {
            assert_eq!(0, 1);
            y
        }
    };

    assert_eq!(l_0, 2);

    assert_eq!(d_0[0], 0x0002A6A5);
    assert_eq!(d_0[1], 0x00024279);

    let mut d_1 = [255_u8; 128];
    let l_1 = super::encode_utf8(&d_0[..l_0], &mut d_1);

    assert_eq!(l_1, 8);

    assert_eq!(d_1[0], 0xF0);
    assert_eq!(d_1[1], 0xAA);
    assert_eq!(d_1[2], 0x9A);
    assert_eq!(d_1[3], 0xA5);
    assert_eq!(d_1[4], 0xF0);
    assert_eq!(d_1[5], 0xA4);
    assert_eq!(d_1[6], 0x89);
    assert_eq!(d_1[7], 0xB9);
}
