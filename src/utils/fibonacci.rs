pub fn fibonacci(n: usize) -> u64 {
    if n == 0 {
        return 0;
    }

    let (mut current, mut next) = (0, 1);

    for _ in 0..(n - 1) {
        (current, next) = (next, current + next);
    }

    next
}

pub fn fibonaccis(n: usize) -> Vec<u32> {
    if n == 0 {
        return vec![0];
    }

    let mut fib_vec: Vec<u32> = vec![0, 1];
    let (mut current, mut next) = (0, 1);

    for _ in 0..(n - 1) {
        (current, next) = (next, current + next);
        fib_vec.push(next);
    }

    fib_vec
}

pub struct Fibonacci {
    minus_1: usize,
    minus_2: usize,
    call_count: usize,
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci {
            minus_1: 1,
            minus_2: 0,
            call_count: 0,
        }
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
            }
            1 => {
                self.call_count += 1;
                return Some(1);
            }
            _ => self.call_count += 1,
        }

        // Calculate values
        let current = self.minus_1 + self.minus_2;
        self.minus_2 = self.minus_1;
        self.minus_1 = current;

        Some(current)
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
        assert_eq!(fibonacci(30), 832040);
        assert_eq!(fibonacci(60), 1548008755920);
        assert_eq!(fibonacci(70), 190392490709135);
        assert_eq!(fibonacci(75), 2111485077978050);
    }

    #[test]
    fn test_fibonaccis() {
        assert_eq!(fibonaccis(0), vec![0]);
        assert_eq!(fibonaccis(1), vec![0, 1]);
        assert_eq!(fibonaccis(2), vec![0, 1, 1]);
        assert_eq!(
            fibonaccis(16),
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987]
        );
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
