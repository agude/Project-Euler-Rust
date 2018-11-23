/*
 * 2520 is the smallest number that can be divided by each of the numbers from
 * 1 to 10 without any remainder.
 *
 * What is the smallest number that is evenly divisible by all of the numbers from 1 to 20?
 */

fn euler_005() -> u32 {
    /*
     * We only need to test a subset of numbers, all other numbers are guaranteed by their
     * inclusion (for example, a number divisible by 5 or by 4 is also divisible by 20).
     *
     * By incrementing by 20 * 19 * 17 * 13 * 11, we insure that the number is divisible by those
     * numbers (and since they are co-prime we don't skip any numbers).
     */
    let divisors = vec![18, 16, 15, 14, 12];
    let increment = 20 * 19 * 17 * 13 * 11;

    let mut test_number = increment;
    loop {
        for &divisor in divisors.iter() {
            // The test number is not evenly divisible, so move on
            if test_number % divisor != 0 {
                test_number += increment;
                break;
            }
            // All test passed, so this number is the one 
            else if divisor == 12 {
                return test_number;
            }
        }
    }
}


fn main() {
    let answer = euler_005();
    println!("{}", answer);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_005() {
        assert_eq!(euler_005(), 232792560);
    }
}
