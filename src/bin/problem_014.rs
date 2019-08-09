/*
 * The following iterative sequence is defined for the set of positive integers:
 *
 *     n --> n/2 (n is even)
 *     n --> 3n + 1 (n is odd)
 *
 * Using the rule above and starting with 13, we generate the following sequence:
 *
 *     13 --> 40 --> 20 --> 10 --> 5 --> 16 --> 8 --> 4 --> 2 --> 1
 *
 * It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
 * Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers
 * finish at 1.
 *
 * Which starting number, under one million, produces the longest chain?
 *
 * NOTE: Once the chain starts the terms are allowed to go above one million.
 */
use std::collections::HashMap;

struct Chain {
    memoized: HashMap<u64, u64>,
}

impl Chain {
    fn new() -> Self {
        let mut memoized: HashMap<u64, u64> = HashMap::new();
        memoized.insert(1, 1);
        return Chain { memoized: memoized };
    }

    fn check_memo(&self, number: u64) -> u64 {
        return match self.memoized.get(&number) {
            Some(&length) => length,
            _             => 0,
        }
    }

    fn unwind_stack(&mut self, base_length: u64, mut stack: Vec<u64>) {
        for stack_position in 0..stack.len() {
            let stack_val = stack.pop().unwrap();
            let insert_length = base_length + 1 + stack_position as u64;
            self.memoized.insert(stack_val, insert_length);
        }
    }

    fn get_length(&mut self, starting_number: u64) -> u64 {
        // First, see if we have the result stored
        let cached_length = Self::check_memo(&self, starting_number);
        if cached_length > 0 {
            return cached_length;
        }
        // Otherwise calculate
        let mut stack: Vec<u64> = vec![starting_number];
        let mut modeable_number = starting_number;

        loop {
            // Compute the new result
            let new_number = match modeable_number % 2 == 0 {
                true => modeable_number / 2,
                false => 3 * modeable_number + 1,
            };

            let new_length = Self::check_memo(&self, new_number);

            if new_length > 0 {
                // We found it! Unwind the stack
                Self::unwind_stack(self, new_length, stack);
                break;
            }
            else {
                stack.push(new_number);
                modeable_number = new_number;
            }
        }
        // Now that we have added the value, return it
        let final_length = Self::check_memo(&self, starting_number);
        return final_length;
    }
}


fn euler_014(max_number: usize) -> u64 {
    let mut chain = Chain::new();
    let mut best_length: u64 = 0;
    let mut best_start: u64 = 0;
    for number in 1..max_number {
        let new_length = chain.get_length(number as u64);
        if new_length >= best_length {
            best_length = new_length;
            best_start = number as u64;
        }
    }
    return best_start;
}


fn main() {
    let answer = euler_014(1_000_000);
    println!("{}", answer);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_014_fast() {
        // Final answer
        assert_eq!(euler_014(20), 19);
        assert_eq!(euler_014(1000), 871);
    }

    #[test]
    fn test_problem_014() {
        // Final answer
        assert_eq!(euler_014(1_000_000), 837_799);
    }
}
