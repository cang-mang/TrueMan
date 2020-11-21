#[test]
fn exe() {
    let src = "汉字".as_bytes();
    assert_eq!(src.len(), 6);
    assert_eq!(src[0], 0xE6);
    assert_eq!(src[1], 0xB1);
    assert_eq!(src[2], 0x89);
    assert_eq!(src[3], 0xE5);
    assert_eq!(src[4], 0xAD);
    assert_eq!(src[5], 0x97);

    let mut d_0 = [65535_u16; 128];
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

    assert_eq!(d_0[0], 0x6C49);
    assert_eq!(d_0[1], 0x5B57);

    let mut d_1 = [255_u8; 128];
    let l_1 = super::encode_utf8(&d_0[..l_0], &mut d_1);

    assert_eq!(l_1, 6);

    assert_eq!(d_1[0], 0xE6);
    assert_eq!(d_1[1], 0xB1);
    assert_eq!(d_1[2], 0x89);
    assert_eq!(d_1[3], 0xE5);
    assert_eq!(d_1[4], 0xAD);
    assert_eq!(d_1[5], 0x97);
}
