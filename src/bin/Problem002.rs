/*
 * Each new term in the Fibonacci sequence is generated by adding the previous two terms. By
 * starting with 1 and 2, the first 10 terms will be:
 *
 *     1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
 *
 * By considering the terms in the Fibonacci sequence whose values do not exceed four million, find
 * the sum of the even-valued terms.
 */
extern crate fibonacci;


fn euler_002(limit: u64) -> u64 {
    let mut n = 0;
    let mut sum = 0;
    loop {
        let current_number = fibonacci::fibonacci_binet(n);
        if current_number > limit {
            break;
        }
        if current_number % 2 == 0 {
            sum += current_number;
        }

        n += 1
    }

    return sum;
}


fn main() {
    let answer = euler_002(4_000_000);
    println!("{}", answer);
}