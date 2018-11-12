pub fn fibonacci(n: u32) -> u32 {
    // Base case
    match n {
        0 => return 0,
        1 => return 1,
        _ => (),
    };

    // Otherwise, recurse
    return fibonacci(n-1) + fibonacci(n-2);
}


pub fn fibonaccis(n: u32) -> Vec<u32> {
    // Base case
    match n {
        0 => return vec![0],
        1 => return vec![0, 1],
        _ => (),
    };

    let mut fib_vec: Vec<u32> = vec![0, 1];
    let mut minus_1 = 1;
    let mut minus_2 = 0;

    for _ in 2..n+1 {
        let current = minus_1 + minus_2;
        fib_vec.push(current);
        minus_2 = minus_1;
        minus_1 = current;
    }

    return fib_vec;
}


pub fn fibonacci_binet(n: u8) -> u64 {
    // Loses accuracy after n = 75
    if n > 75 {
        panic!("results are inaccurate for n greater than 75")
    }

    let sqrt5: f64 = (5.0 as f64).sqrt();
    let phi: f64   = (1. + sqrt5) / 2.;

    let phi_n = phi.powi(n as i32);

    let result = (phi_n / sqrt5).round() as u64;

    return result;
}


pub struct Fibonacci {
    minus_1: usize,
    minus_2: usize,
    call_count: usize,
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        return Fibonacci{
            minus_1: 1,
            minus_2: 0,
            call_count: 0,
        };
    }
}

impl Iterator for Fibonacci {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        // We handle the first two cases with hardcoded values, all the others we derive.
        match self.call_count {
            0 => {
                self.call_count += 1;
                return Some(0);
            },
            1 => {
                self.call_count += 1;
                return Some(1);
            },
            _ => self.call_count += 1,
        }

        // Calculate values
        let current = self.minus_1 + self.minus_2;
        self.minus_2 = self.minus_1;
        self.minus_1 = current;

        return Some(current);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(16), 987);
        assert_eq!(fibonacci(30), 832040);
    }

    #[test]
    fn test_fibonaccis() {
        assert_eq!(fibonaccis(0), vec![0]);
        assert_eq!(fibonaccis(1), vec![0, 1]);
        assert_eq!(fibonaccis(2), vec![0, 1, 1]);
        assert_eq!(fibonaccis(16), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987]);
    }

    #[test]
    fn test_fibonacci_binet() {
        assert_eq!(fibonacci_binet(0), 0);
        assert_eq!(fibonacci_binet(1), 1);
        assert_eq!(fibonacci_binet(2), 1);
        assert_eq!(fibonacci_binet(10), 55);
        assert_eq!(fibonacci_binet(30), 832040);
        assert_eq!(fibonacci_binet(60), 1548008755920);
        assert_eq!(fibonacci_binet(70), 190392490709135);
        assert_eq!(fibonacci_binet(75), 2111485077978050);
    }

    #[test]
    #[should_panic]
    fn test_invalid_fibonacci_binet() {
        fibonacci_binet(76);
    }

    #[test]
    fn test_fibonacci_iter() {
        let truth = vec![
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
            6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040,
            1346269, 2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986,
            102334155,
        ];
        let fib = Fibonacci::new();
        for (i, val) in fib.take(truth.len()).enumerate() {
            println!("{}: {}", i, val);
            assert_eq!(truth[i], val);
        }
    }
}
