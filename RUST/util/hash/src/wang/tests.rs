#[test]
fn exe_direct() {
    let x = 9876543210123456789_u64;
    let y = super::direct(x);

    assert_eq!(y, 14255830987705448333);
}
/*----------------------------------------------------------------------------*/
#[test]
fn exe_inverse() {
    let x = 14255830987705448333_u64;
    let y = super::inverse(x);

    assert_eq!(y, 9876543210123456789);
}
