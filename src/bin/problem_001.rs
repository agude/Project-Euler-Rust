/*
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
 * The sum of these multiples is 23.
 *
 * Find the sum of all the multiples of 3 or 5 below 1000.
 */

fn euler_001(max: u32) -> u32 {

    let mut sum: u32 = 0;
    for test_number in 1..max {
        // Check if the number is divisible by 3 or 5
        let div_3: bool = { test_number % 3 == 0 };
        let div_5: bool = { test_number % 5 == 0 };

        // If divisible, add the numbers
        match div_3 | div_5 {
            true  => sum += test_number,
            false => continue,
        }
    }

    return sum;
}


fn main() {
    let answer = euler_001(1000);
    println!("{}", answer);
}
