pub fn luhn(cc_numbers: &str) -> bool {
    let non_white_spaces: Vec<char> = cc_numbers.chars().filter(|c| c != &' ').collect();

    if non_white_spaces.len() < 2
        || !non_white_spaces
            .clone()
            .into_iter()
            .all(|c| c.is_ascii_digit())
    {
        return false;
    }

    let result: u32 = non_white_spaces
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let mut num = c.to_digit(10).unwrap();

            if i % 2 == 1 {
                num *= 2;

                if num > 9 {
                    num = (num / 10) + (num % 10);
                }
            }

            num
        })
        .sum();

    result % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

fn main() {}
