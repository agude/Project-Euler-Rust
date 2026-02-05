/*
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any
 *      remainder.
 *
 * What is the smallest number that is evenly divisible by all of the numbers from 1 to 20?
 */

fn euler_006() -> u32 {
    // ( 1 + 2 + ... + 100)^2
    let sum: u32 = (1..=100).sum();
    let sum_squared: u32 = sum.pow(2);

    // ( 1^2 + 2^2 + ... + 100^2)
    let squared_sum: u32 = (1..=100).map(|x| (x as u32).pow(2)).sum();

    return sum_squared - squared_sum;
}


fn main() {
    let answer = euler_006();
    println!("{}", answer);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_006() {
        assert_eq!(euler_006(), 25164150);
    }
}
