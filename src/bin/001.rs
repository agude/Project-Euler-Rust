/*
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
 * The sum of these multiples is 23.
 *
 * Find the sum of all the multiples of 3 or 5 below 1000.
 */

fn euler_001(max: u32) -> u32 {

    let sum: u32 =  (1..max)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .sum();

    return sum;
}


fn main() {
    let answer = euler_001(1000);
    println!("{}", answer);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_001_example() {
        // From example
        assert_eq!(euler_001(10), 23);
    }

    #[test]
    fn test_problem_001() {
        // Final answer
        assert_eq!(euler_001(1000), 233168);
    }
}
