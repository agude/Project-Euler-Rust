/*
 * The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle
 * number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28.  The first ten terms would be:
 *
 *     1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
 *
 * Let us list the factors of the first seven triangle numbers:
 *
 *      1: 1
 *      3: 1,3
 *      6: 1,2,3,6
 *     10: 1,2,5,10
 *     15: 1,3,5,15
 *     21: 1,3,7,21
 *     28: 1,2,4,7,14,28
 *
 * We can see that 28 is the first triangle number to have over five divisors.
 *
 * What is the value of the first triangle number to have over five hundred divisors?
 */
extern crate polygonal;
extern crate factorization;

fn euler_012(number: usize) -> i64 {
    let mut triangulars: polygonal::Polygonals = polygonal::get_triangulars_iterator(0);
    return triangulars.find(|&x| factorization::number_of_factors(x as u64) > number).unwrap();
}


fn main() {
    let answer = euler_012(500);
    println!("{}", answer);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_012() {
        // Final answer
        assert_eq!(euler_012(500), 76_576_500);
    }
}
