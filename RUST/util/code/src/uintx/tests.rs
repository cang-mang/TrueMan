use super::Code as _Code;
/*============================================================================*/
#[test]
fn exe() {
    let val_128 = 0x_12_34_56_78_90_AB_CD_EF_0A_1B_2C_3D_4E_5F_68_79_u128;
    let val_64  = val_128 as u64;

    let mut buf = [255_u8; 23];
    val_128.encode_be(&mut buf);
    assert_eq!(buf[ 0], 0x00);
    assert_eq!(buf[ 1], 0x00);
    assert_eq!(buf[ 2], 0x00);
    assert_eq!(buf[ 3], 0x00);
    assert_eq!(buf[ 4], 0x00);
    assert_eq!(buf[ 5], 0x00);
    assert_eq!(buf[ 6], 0x00);
    assert_eq!(buf[ 7], 0x12);
    assert_eq!(buf[ 8], 0x34);
    assert_eq!(buf[ 9], 0x56);
    assert_eq!(buf[10], 0x78);
    assert_eq!(buf[11], 0x90);
    assert_eq!(buf[12], 0xAB);
    assert_eq!(buf[13], 0xCD);
    assert_eq!(buf[14], 0xEF);
    assert_eq!(buf[15], 0x0A);
    assert_eq!(buf[16], 0x1B);
    assert_eq!(buf[17], 0x2C);
    assert_eq!(buf[18], 0x3D);
    assert_eq!(buf[19], 0x4E);
    assert_eq!(buf[20], 0x5F);
    assert_eq!(buf[21], 0x68);
    assert_eq!(buf[22], 0x79);

    let mut val = 0_u128;
    val.decode_le(&buf);
    assert_eq!(val, 0x0AEFCDAB907856341200000000000000);

    val_128.encode_le(&mut buf);
    assert_eq!(buf[ 0], 0x79);
    assert_eq!(buf[ 1], 0x68);
    assert_eq!(buf[ 2], 0x5F);
    assert_eq!(buf[ 3], 0x4E);
    assert_eq!(buf[ 4], 0x3D);
    assert_eq!(buf[ 5], 0x2C);
    assert_eq!(buf[ 6], 0x1B);
    assert_eq!(buf[ 7], 0x0A);
    assert_eq!(buf[ 8], 0xEF);
    assert_eq!(buf[ 9], 0xCD);
    assert_eq!(buf[10], 0xAB);
    assert_eq!(buf[11], 0x90);
    assert_eq!(buf[12], 0x78);
    assert_eq!(buf[13], 0x56);
    assert_eq!(buf[14], 0x34);
    assert_eq!(buf[15], 0x12);
    assert_eq!(buf[16], 0x00);
    assert_eq!(buf[17], 0x00);
    assert_eq!(buf[18], 0x00);
    assert_eq!(buf[19], 0x00);
    assert_eq!(buf[20], 0x00);
    assert_eq!(buf[21], 0x00);
    assert_eq!(buf[22], 0x00);

    val = 0_u128;
    val.decode_be(&buf);
    assert_eq!(val, 0x0AEFCDAB907856341200000000000000);

    let mut buf = [255_u8; 7];
    val_64.encode_be(&mut buf);
    assert_eq!(buf[0], 0x1B);
    assert_eq!(buf[1], 0x2C);
    assert_eq!(buf[2], 0x3D);
    assert_eq!(buf[3], 0x4E);
    assert_eq!(buf[4], 0x5F);
    assert_eq!(buf[5], 0x68);
    assert_eq!(buf[6], 0x79);

    let mut val = 0_u64;
    val.decode_le(&buf);
    assert_eq!(val, 0x0079685F4E3D2C1B);

    val_64.encode_le(&mut buf);
    assert_eq!(buf[0], 0x79);
    assert_eq!(buf[1], 0x68);
    assert_eq!(buf[2], 0x5F);
    assert_eq!(buf[3], 0x4E);
    assert_eq!(buf[4], 0x3D);
    assert_eq!(buf[5], 0x2C);
    assert_eq!(buf[6], 0x1B);

    val = 0_u64;
    val.decode_be(&buf);
    assert_eq!(val, 0x0079685F4E3D2C1B);
}
