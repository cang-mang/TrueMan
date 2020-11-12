#[test]
fn exe_x32() {
    let key = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^".as_bytes();

    let x = super::x32(key);
    assert_eq!(x, 523317554);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_x64() {
    let key = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^".as_bytes();

    let x = super::x64(key);
    assert_eq!(x, 11266796566932084146);
}
