use std::io::Read;

#[test]
fn test() {
    const TEST: &str = " t/prau0INWoWUQ0LgQ\tMUdJRcCetBZP\nAyD+DCpn 01yWZT/LBo3Ogk0INwwuAtKNI ";

    let mut stripped = String::new();
    super::WhitespaceRemover::new(TEST.as_bytes())
        .read_to_string(&mut stripped)
        .unwrap();

    assert_eq!(
        stripped,
        "t/prau0INWoWUQ0LgQMUdJRcCetBZPAyD+DCpn01yWZT/LBo3Ogk0INwwuAtKNI"
    );
}

#[test]
fn long_whitespace() {
    let mut test_string = "a".to_owned();
    test_string.reserve_exact(161);
    for _ in 0..20 {
        test_string.push_str("        ");
    }
    test_string.push('b');

    let mut stripped = String::new();
    super::WhitespaceRemover::new(test_string.as_bytes())
        .read_to_string(&mut stripped)
        .unwrap();

    assert_eq!(stripped, "ab");
}
