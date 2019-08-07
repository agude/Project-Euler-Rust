pub fn proper_factors(number: u64) -> Vec<u64> {
    return (1..number).filter(|&x| number % x == 0).collect();
}

pub fn number_of_proper_factors(number: u64) -> usize {
    return proper_factors(number).len();
}

pub fn number_of_factors(number: u64) -> usize {
    return number_of_proper_factors(number) + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proper_factors_large() {
        let truth = vec![
            1, 2, 4, 5, 8, 10, 16, 20, 25, 40, 50, 80, 100, 125, 200, 250, 400, 500, 625, 1000,
            1250, 2000, 2500, 5000,
        ];
        assert_eq!(proper_factors(10000), truth);
    }

    #[test]
    fn test_proper_factors_prime() {
        let truth = vec![1];
        assert_eq!(proper_factors(1609), truth);
    }

    #[test]
    fn test_number_of_proper_factors_large() {
        let truth = vec![
            1, 2, 4, 5, 8, 10, 16, 20, 25, 40, 50, 80, 100, 125, 200, 250, 400, 500, 625, 1000,
            1250, 2000, 2500, 5000,
        ];
        assert_eq!(number_of_proper_factors(10000), truth.len());
    }

    #[test]
    fn test_number_of_proper_factors_prime() {
        let truth = vec![1];
        assert_eq!(number_of_proper_factors(1609), truth.len());
    }

    #[test]
    fn test_number_of_factors_large() {
        let truth = vec![
            1, 2, 4, 5, 8, 10, 16, 20, 25, 40, 50, 80, 100, 125, 200, 250, 400, 500, 625, 1000,
            1250, 2000, 2500, 5000,
        ];
        assert_eq!(number_of_factors(10000), truth.len() + 1);
    }

    #[test]
    fn test_number_of_factors_prime() {
        let truth = vec![1];
        assert_eq!(number_of_factors(1609), truth.len() + 1);
    }
}
