#[test]
fn exe() {
    let src = &[
        b'Q', b'u', b'o', b't', b'e', b'd', b'-', b'p',
        b'r', b'i', b'n', b't', b'a', b'b', b'l', b'e'
    ];

    let mut d_0 = [255_u8; 128];
    let l_0 = super::encode(src, &mut d_0);

    assert_eq!(l_0, 24);

    assert_eq!(d_0[ 0], b'U');
    assert_eq!(d_0[ 1], b'X');
    assert_eq!(d_0[ 2], b'V');
    assert_eq!(d_0[ 3], b'v');
    assert_eq!(d_0[ 4], b'd');
    assert_eq!(d_0[ 5], b'G');
    assert_eq!(d_0[ 6], b'V');
    assert_eq!(d_0[ 7], b'k');
    assert_eq!(d_0[ 8], b'L');
    assert_eq!(d_0[ 9], b'X');
    assert_eq!(d_0[10], b'B');
    assert_eq!(d_0[11], b'y');
    assert_eq!(d_0[12], b'a');
    assert_eq!(d_0[13], b'W');
    assert_eq!(d_0[14], b'5');
    assert_eq!(d_0[15], b'0');
    assert_eq!(d_0[16], b'Y');
    assert_eq!(d_0[17], b'W');
    assert_eq!(d_0[18], b'J');
    assert_eq!(d_0[19], b's');
    assert_eq!(d_0[20], b'Z');
    assert_eq!(d_0[21], b'Q');
    assert_eq!(d_0[22], b'=');
    assert_eq!(d_0[23], b'=');

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

    assert_eq!(d_1[ 0], b'Q');
    assert_eq!(d_1[ 1], b'u');
    assert_eq!(d_1[ 2], b'o');
    assert_eq!(d_1[ 3], b't');
    assert_eq!(d_1[ 4], b'e');
    assert_eq!(d_1[ 5], b'd');
    assert_eq!(d_1[ 6], b'-');
    assert_eq!(d_1[ 7], b'p');
    assert_eq!(d_1[ 8], b'r');
    assert_eq!(d_1[ 9], b'i');
    assert_eq!(d_1[10], b'n');
    assert_eq!(d_1[11], b't');
    assert_eq!(d_1[12], b'a');
    assert_eq!(d_1[13], b'b');
    assert_eq!(d_1[14], b'l');
    assert_eq!(d_1[15], b'e');
}
