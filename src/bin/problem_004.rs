/*
 * A palindromic number reads the same both ways. The largest palindrome made
 * from the product of two 2-digit numbers is 9009 = 91 x 99.
 *
 * Find the largest palindrome made from the product of two 3-digit numbers.
 *
 */
extern crate palindromic;


fn euler_004() -> u32 {
    let mut current_palindrome: u32 = 0;
    for i in 100..999 {
        for j in 100..999 {
            let test_number = i * j;
            if test_number > current_palindrome {
                if palindromic::is_palindromic(test_number as u64) {
                    current_palindrome = test_number;
                }
            }
        }
    }

    return current_palindrome;
}


fn main() {
    let answer = euler_004();
    println!("{}", answer);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_004() {
        assert_eq!(euler_004(), 906609);
    }
}
