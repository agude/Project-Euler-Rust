pub fn prime_sieve(max_number: u64) -> Vec<u64> {
    if max_number < 2 {
        return Vec::new();
    };

    // Set up the sieve and output
    let mut is_prime: Vec<bool> = vec![true; (max_number + 1) as usize];
    let mut output_primes: Vec<u64> = Vec::new();

    // Handle small numbers
    if max_number == 2 {
        return vec![2];
    };
    is_prime[0] = false;
    is_prime[1] = false;

    // Sieving the numbers
    let end = is_prime.len();
    for prime_to_check in 2..end {
        if is_prime[prime_to_check] {
            // Save the value
            output_primes.push(prime_to_check as u64);

            // We can start at i*i and not just 2*i, because lower multiples have already been
            // removed.
            let start = prime_to_check * prime_to_check;
            for mask in (start..end).step_by(prime_to_check) {
                is_prime[mask] = false;
            }
        }
    }

    output_primes
}

pub fn prime_factors(number: u64) -> Vec<u64> {
    // Handle small numbers
    match number {
        0 => return vec![],
        1 => return vec![],
        2 => return vec![2],
        _ => (),
    };

    let mut factors: Vec<u64> = Vec::new();
    let mut div_number = number;
    // We divide through by primes until we reach 1, repeating with each prime until it no longer
    // divides through evenly.
    let primes = Primes::new();
    for prime in primes {
        while div_number.is_multiple_of(prime) {
            div_number /= prime;
            factors.push(prime);

            // We have divided out all the primes
            if div_number <= 1 {
                return factors;
            }
        }
        // If our number is prime, we only need to test up to half its value.
        let beyond_sqrt = prime as f64 > (number as f64).sqrt();
        if factors.is_empty() && beyond_sqrt {
            return vec![number];
        }
    }

    factors
}

pub struct Primes {
    first_run: bool,
    not_primes: std::collections::HashMap<u64, u64>,
    current_number: u64,
}

impl Default for Primes {
    fn default() -> Self {
        Self::new()
    }
}

impl Primes {
    pub fn new() -> Primes {
        Primes {
            first_run: true,
            not_primes: std::collections::HashMap::new(),
            current_number: 1,
        }
    }
}

impl Iterator for Primes {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        // We put the only even prime, 2, in by hand to simplify the algorithm
        if self.first_run {
            self.first_run = false;
            return Some(2);
        }

        // Otherwise iterate over odd numbers starting at 3
        loop {
            // We're imitating a for loop, and it's much easier to increment at the start instead
            // of one of the multiple exit points.
            self.current_number += 2;

            let root_prime: u64 = match self.not_primes.get(&self.current_number) {
                Some(&x) => x,
                None => 0,
            };

            // Our number is prime, since it's not in the not_primes hashmap
            if root_prime == 0 {
                self.not_primes.insert(
                    self.current_number * self.current_number,
                    self.current_number,
                );
                return Some(self.current_number);
            } else {
                // Out number is not prime, so add some more masks to the not_primes hashmap.
                let mut k = self.current_number + root_prime;
                loop {
                    let k_not_prime = self.not_primes.contains_key(&k);
                    let k_is_even = k.is_multiple_of(2);
                    if k_not_prime || k_is_even {
                        k += root_prime;
                    } else {
                        break;
                    }
                }
                self.not_primes.insert(k, root_prime);
            }
        }
    }
}

#[allow(dead_code)]
fn is_prime(input: u64) -> bool {
    if input < 2 {
        // 0, 1 and negative are not prime
        false
    } else if input < 4 {
        // 3 is prime
        true
    } else if input.is_multiple_of(2) {
        // even numbers are not prime
        false
    } else if input < 9 {
        // 6, 8 has been removed above
        true
    } else if input.is_multiple_of(3) {
        // numbers divisible by 3 are not prime
        false
    } else {
        let floor_root: u64 = (input as f64).sqrt().floor() as u64;
        let mut f: u64 = 5;

        while f <= floor_root {
            if input.is_multiple_of(f) || input.is_multiple_of(f + 2) {
                return false;
            }
            f += 6;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large_prime_sieve() {
        // Correctly generates a list of primes
        let big_primes = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541,
        ];
        assert_eq!(prime_sieve(541), big_primes);
    }

    #[test]
    fn test_small_prime_sieve() {
        // Works for small numbers as well, where we have some hardcoded results
        assert_eq!(prime_sieve(0), vec![]);
        assert_eq!(prime_sieve(2), vec![2]);
        assert_eq!(prime_sieve(3), vec![2, 3]);
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(0), vec![]);
        assert_eq!(prime_factors(1), vec![]);
        assert_eq!(prime_factors(2), vec![2]);
        assert_eq!(prime_factors(3), vec![3]);
        assert_eq!(prime_factors(100), vec![2, 2, 5, 5]);
        assert_eq!(prime_factors(137), vec![137]);
        assert_eq!(prime_factors(123345), vec![3, 3, 5, 2741]);
    }

    #[test]
    fn test_prime_iter() {
        let truth = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541,
        ];
        let primes = Primes::new();
        for (i, val) in primes.take(truth.len()).enumerate() {
            println!("{}: {}", i, val);
            assert_eq!(truth[i], val);
        }
    }

    #[test]
    fn test_is_prime_true() {
        let truth = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541, 179425027, 179425579, 179426081, 179426549,
        ];
        for val in truth {
            assert_eq!(is_prime(val), true);
        }
    }

    #[test]
    fn test_is_prime_false() {
        let truth = vec![
            0, 1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28, 30, 32, 33,
            34, 35, 36, 38, 39, 40, 42, 44, 45, 46, 48, 49, 50, 51, 52, 54, 55, 56, 57, 58, 60, 62,
            63, 104719, 104724, 179425028, 179425029,
        ];
        for val in truth {
            assert_eq!(is_prime(val), false);
        }
    }
}
