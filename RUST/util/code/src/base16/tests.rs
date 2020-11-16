#[test]
fn exe() {
    let src = &[
        0x04u8, 0x40, 0x40, 0x09, 0xd3, 0x1a, 0xfc, 0x28,
        0x45  , 0x2a, 0x8b, 0x4d, 0x9e, 0x64, 0x71, 0x04
    ];

    let mut d_0 = [255_u8; 128];
    let l_0 = super::encode(src, &mut d_0, true);

    assert_eq!(l_0, 32);

    assert_eq!(d_0[ 0], b'0');
    assert_eq!(d_0[ 1], b'4');
    assert_eq!(d_0[ 2], b'4');
    assert_eq!(d_0[ 3], b'0');
    assert_eq!(d_0[ 4], b'4');
    assert_eq!(d_0[ 5], b'0');
    assert_eq!(d_0[ 6], b'0');
    assert_eq!(d_0[ 7], b'9');
    assert_eq!(d_0[ 8], b'D');
    assert_eq!(d_0[ 9], b'3');
    assert_eq!(d_0[10], b'1');
    assert_eq!(d_0[11], b'A');
    assert_eq!(d_0[12], b'F');
    assert_eq!(d_0[13], b'C');
    assert_eq!(d_0[14], b'2');
    assert_eq!(d_0[15], b'8');
    assert_eq!(d_0[16], b'4');
    assert_eq!(d_0[17], b'5');
    assert_eq!(d_0[18], b'2');
    assert_eq!(d_0[19], b'A');
    assert_eq!(d_0[20], b'8');
    assert_eq!(d_0[21], b'B');
    assert_eq!(d_0[22], b'4');
    assert_eq!(d_0[23], b'D');
    assert_eq!(d_0[24], b'9');
    assert_eq!(d_0[25], b'E');
    assert_eq!(d_0[26], b'6');
    assert_eq!(d_0[27], b'4');
    assert_eq!(d_0[28], b'7');
    assert_eq!(d_0[29], b'1');
    assert_eq!(d_0[30], b'0');
    assert_eq!(d_0[31], b'4');

    let mut d_1 = [255_u8; 128];
    let l_1 = match super::decode(&d_0[..l_0], &mut d_1) {
        Ok(y)  => {
            y
        }
        Err(y) => {
            assert_eq!(0, 1);
            y
        }
    };

    assert_eq!(l_1, 16);

    assert_eq!(d_1[ 0], 0x04);
    assert_eq!(d_1[ 1], 0x40);
    assert_eq!(d_1[ 2], 0x40);
    assert_eq!(d_1[ 3], 0x09);
    assert_eq!(d_1[ 4], 0xD3);
    assert_eq!(d_1[ 5], 0x1A);
    assert_eq!(d_1[ 6], 0xFC);
    assert_eq!(d_1[ 7], 0x28);
    assert_eq!(d_1[ 8], 0x45);
    assert_eq!(d_1[ 9], 0x2A);
    assert_eq!(d_1[10], 0x8B);
    assert_eq!(d_1[11], 0x4D);
    assert_eq!(d_1[12], 0x9E);
    assert_eq!(d_1[13], 0x64);
    assert_eq!(d_1[14], 0x71);
    assert_eq!(d_1[15], 0x04);
}
