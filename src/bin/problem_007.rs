/*
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is
 * 13.
 *
 * What is the 10,001st prime number?
 */
extern crate primes;


fn euler_007(nth_prime: usize) -> u64 {
    match primes::Primes::new().nth(nth_prime) {
        Some(x) => x,
        None    => 0,
    }
}


fn main() {
    let answer = euler_007(10_001);
    println!("{}", answer);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_007() {
        assert_eq!(euler_007(10_001), 104759);
    }
}
