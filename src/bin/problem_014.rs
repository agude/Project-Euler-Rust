/*
 * The following iterative sequence is defined for the set of positive integers:
 *
 * n --> n/2 (n is even)
 * n --> 3n + 1 (n is odd)
 *
 * Using the rule above and starting with 13, we generate the following sequence:
 * 13 --> 40 --> 20 --> 10 --> 5 --> 16 --> 8 --> 4 --> 2 --> 1
 *
 * It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
 * Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers
 * finish at 1.
 *
 * Which starting number, under one million, produces the longest chain?
 *
 * NOTE: Once the chain starts the terms are allowed to go above one million.
 */


fn chain_length(number: u64) -> u64 {
    let mut chain_length = 1;
    let mut mod_number = number;

    // If number is even, n --> n/2;
    // else n --> 3n+1
    while mod_number > 1 {
        chain_length += 1;
        if mod_number % 2 == 0 {
            mod_number = mod_number / 2;
        } else {
            mod_number = 3 * mod_number + 1;
        }
    }

    return chain_length;
}


fn euler_014() -> u64 {
    // Find the largest value
    let mut longest_start = 0;
    let mut longest_chain_length = 0;

    for i in 1..1_000_000 {
        let new_chain_length = chain_length(i);
        if new_chain_length > longest_chain_length {
            longest_start = i;
            longest_chain_length = new_chain_length;
        }
    }

    return longest_start;
}


fn main() {
    let answer = euler_014();
    println!("{}", answer);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_014() {
        // Final answer
        assert_eq!(euler_014(), 837799);
    }
}
