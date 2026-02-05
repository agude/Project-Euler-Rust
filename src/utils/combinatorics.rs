use std::cmp::max;
use std::cmp::min;

pub fn n_choose_k(n: u32, k: u32) -> u128 {
    // We define n choose k for k > n as 0, that is, there are no way to choose more items than
    // exist in a set.
    if k > n {
        return 0;
    }
    // There is only one way to pick all the objects from a set.
    if k == n {
        return 1;
    }

    // Factorial functions overflow very easily, so we cancel first
    let bottom = max(k, n - k);
    let other = min(k, n - k);

    let ratio = factorial_ratio(n, bottom);
    let denominator = factorial(other);

    ratio / denominator
}

pub fn factorial_ratio(top: u32, bottom: u32) -> u128 {
    if top > bottom {
        let start = (bottom + 1) as u128;
        return (start..=top as u128).product();
    }
    0
}

pub fn factorial(n: u32) -> u128 {
    match n {
        0 => 1,
        _ => (1..=n as u128).product(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(
            factorial(34),
            295_232_799_039_604_140_847_618_609_643_520_000_000
        );
    }

    #[test]
    fn test_n_choose_k() {
        assert_eq!(n_choose_k(0, 0), 1);
        assert_eq!(n_choose_k(1, 1), 1);
        assert_eq!(n_choose_k(5, 3), 10);
        assert_eq!(n_choose_k(0, 1), 0);
        assert_eq!(n_choose_k(34, 17), 2_333_606_220);
    }

    #[test]
    fn test_factorial_ratio() {
        assert_eq!(factorial_ratio(10, 5), 30_240);
        assert_eq!(factorial_ratio(1000, 999), 1000);
        assert_eq!(factorial_ratio(1, 100), 0);
    }
}
