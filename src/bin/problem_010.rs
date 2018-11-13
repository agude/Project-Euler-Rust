/*
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 *
 * Find the sum of all the primes below two million.
 */
extern crate primes;


fn euler_010(max_prime: u64) -> u64 {
    let answer = primes::prime_sieve(max_prime).iter().sum();
    return answer;
}


fn main() {
    let answer = euler_010(2_000_000);
    println!("{}", answer);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_010_example() {
        // From example
        assert_eq!(euler_010(10), 17);
    }

    #[test]
    fn test_problem_010() {
        // Final answer
        assert_eq!(euler_010(2_000_000), 142913828922);
    }
}
