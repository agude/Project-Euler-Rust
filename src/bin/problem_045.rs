/*
 * Triangle, pentagonal, and hexagonal numbers are generated by the following formulae:
 *
 * Triangle        Tn = n(n+1)/2      1, 3, 6, 10, 15, ...
 * Pentagonal      Pn = n(3n-1)/2     1, 5, 12, 22, 35, ...
 * Hexagonal       Hn = n(2n-1)       1, 6, 15, 28, 45, ...
 *
 * It can be verified that T285 = P165 = H143 = 40755.
 *
 * Find the next triangle number that is also pentagonal and hexagonal.
 */
extern crate polygonal;


fn euler_045() -> u64 {
    /*
     * We generate the hexagonal numbers using the formula provided: 
     *
     *   n*(2*n-1)
     * 
     * We then check if they are pentagonal, but we do not check if they are triangular as all
     * hexagonal numbers are by definition. We generate hexagonal numbers because there are fewer
     * of them, and therefore we have to check fewer numbers before finding the correct result.
     */
    let mut n: u64 = 144; // H143 satisfies the conditions, so we start after

    // Try hexagonal numbers until we find a match
    loop {
        let num = n * (2 * n - 1);
        // If it is also pentagonal, we're done
        if polygonal::is_pentagonal(num) {
            return num;
        }
        else {
            n += 1;
        }
    }
}


fn main() {
    let answer = euler_045();
    println!("{}", answer);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_045() {
        assert_eq!(euler_045(), 1_533_776_805);
    }
}
