use super::*;

#[test]
fn status_ordering() {
    assert!(Status::Error > Status::Warn);
    assert!(Status::Warn > Status::Ok);
}
