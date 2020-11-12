#[test]
fn exe_x32() {
    let key = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^".as_bytes();

    let x = super::x32(key);
    assert_eq!(x, 4240981924);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_x64() {
    let key = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^".as_bytes();

    let x = super::x64(key);
    assert_eq!(x, 7619294831832707076);
}
