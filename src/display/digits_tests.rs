use super::table::digits;
#[test]
fn digits_zero() {
    assert_eq!(digits(0), 1);
}
#[test]
fn digits_single_digit() {
    assert_eq!(digits(9), 1);
}
#[test]
fn digits_two_digits() {
    assert_eq!(digits(10), 2);
}
#[test]
fn digits_large() {
    assert_eq!(digits(1000), 4);
}
