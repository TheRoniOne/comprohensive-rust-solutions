pub fn collatz_length(mut n: u32) -> u32 {
    let mut length = 1;

    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
            length += 1;
        } else {
            n = 3 * n + 1;
            length += 1;
        }
    }

    length
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}
