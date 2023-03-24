// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let cc = cc_number.chars().filter(|&x| x != ' ').rev().enumerate();

    for (idx, digit) in cc {
        if idx % 2 != 0 {
            let num = digit.to_digit(10).unwrap() * 2;

            if num > 9 {
                let num_str: Vec<_> = num.to_string().chars().map(|x| x.to_digit(10).unwrap()).collect();


                for digit in num_str {
                    sum += digit;
                }
                continue;
            }

            sum += num;
            continue;
        }

        sum += digit.to_digit(10).unwrap();
    }

    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
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
