#[test]
fn exe_x32() {
    let key = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^".as_bytes();

    let x = super::x32(key, 0x_FFFFFFFF_u32);
    assert_eq!(x, 2599008376);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_x64() {
    let key = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^".as_bytes();

    let x = super::x64(key, 0x_FFFFFFFF_FFFFFFFF_u64);
    assert_eq!(x, 17226521755326120047);
}
