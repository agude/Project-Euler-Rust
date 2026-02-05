fn reverse(input: u64) -> u64 {
    let mut reversed: u64 = 0;
    let mut number = input;

    while number > 0 {
        reversed = 10 * reversed + number % 10;
        number = number / 10;
    }

    reversed
}

pub fn is_palindromic(input: u64) -> bool {
    input == reverse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(0), 0);
        assert_eq!(reverse(1), 1);
        assert_eq!(reverse(10), 1);
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn test_is_palindromic_true() {
        assert!(is_palindromic(1));
        assert!(is_palindromic(123321));
        assert!(is_palindromic(12321));
    }

    #[test]
    fn test_is_palindromic_false() {
        assert_eq!(is_palindromic(10), false);
        assert_eq!(is_palindromic(123), false);
    }
}

