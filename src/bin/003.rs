/*
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * 
 * What is the largest prime factor of the number 600851475143?
 */
use euler_rust::utils::primes;


fn euler_003(input: u64) -> u64 {
    let factors = primes::prime_factors(input);
    let largest_factor = match factors.last() {
        Some(&x) => x,
        None     => 0,
    };

    return largest_factor;
}


fn main() {
    let answer = euler_003(600_851_475_143);
    println!("{}", answer);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_003() {
        assert_eq!(euler_003(600_851_475_143), 6857);
    }
}
