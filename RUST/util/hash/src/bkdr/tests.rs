#[test]
fn exe_x_0() {
    let key = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^".as_bytes();

    let x = super::x_0(key, 131, 0);
    assert_eq!(x, 3255416723);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_time33() {
    let key = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^".as_bytes();

    let x = super::time33(key, 0);
    assert_eq!(x, 2622491663);
}
