/*
 * A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a**2 + b**2 =
 * c**2
 *
 * For example, 3**2 + 4**2 = 9 + 16 = 25 = 5**2.
 *
 * There exists exactly one Pythagorean triplet for which a + b + c = 1000. Find the product abc.
 */

fn euler_009(target: u32) -> (u32, u32, u32) {
    // We make all possible Pythagorean triplets that satisfy the conditions
    let c_limit: u32 = target - 3;
    for c in 1..=c_limit {
        // Calculate the new b limit
        let b_limit = target - c - 1;
        for b in 1..=b_limit {
            // Compute a
            let a = target - b - c;

            // Test if we have a valid triplet
            if (a * a) + (b * b) == (c * c) {
                return (a, b, c);
            }
        }
    }

    (0, 0, 0)
}

fn main() {
    let (a, b, c) = euler_009(1000);
    println!("{} {} {}", a, b, c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_009() {
        assert_eq!(euler_009(1000), (375, 200, 425));
    }
}
