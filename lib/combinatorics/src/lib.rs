pub fn n_choose_k(n: u32, k: u32) -> u128 {
    // We define n choose k for k > n as 0, that is, there are no way to choose more items than
    // exist in a set.
    if k > n {
        return 0;
    }

    let numerator = factorial(n);
    let denominator = factorial(k) * factorial(n-k);

    return numerator / denominator;
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
}
