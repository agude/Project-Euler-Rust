/*
 * Starting in the top left corner of a 2x2 grid, and only being able to move to the right and
 * down, there are exactly 6 routes to the bottom right corner.
 *
 * How many such routes are there through a 20x20 grid?
 */
extern crate combinatorics;


fn euler_015(max_num: u32) -> u128 {
    /*
     * We notice that we can encode a path as a binary number, where 1 means "right" and 0 means
     * "down". Then paths must be of length 2n, and there must be n 1s and n 0s. Then to specify a
     * path we just need to specify the location of the 1s. The number of ways we can do this is 2n
     * choose n.
     */
    return combinatorics::n_choose_k(2 * max_num, max_num);
}


fn main() {
    let answer = euler_015(20);
    println!("{}", answer);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_015() {
        // Final answer
        assert_eq!(euler_015(20), 137_846_528_820);
    }
}
