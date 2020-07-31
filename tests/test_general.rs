#[test]
fn test_get_version() {
    assert_eq!(&configster::get_ver()[..3], "0.1");
}
