fn is_polygonal(number: u64, multiplier: u64, divisor: u64) -> bool {
    let sqrt_test = (((multiplier * number + 1) as f64).sqrt()) + 1.0;
    let test_num = sqrt_test / divisor as f64;

    // If the test_num is positive and integral, the number is polygonal
    return test_num.is_sign_positive() & (test_num.floor()  == test_num);
}


pub fn is_triangular(number: u64) -> bool {
    return is_polygonal(number, 8, 2);
}


pub fn is_pentagonal(number: u64) -> bool {
    return is_polygonal(number, 24, 6);
}


pub fn is_hexagonal(number: u64) -> bool {
    return is_polygonal(number, 8, 4);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_is_triangular_true() {
        let truth = vec![
            0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210,
            231, 253, 276, 300, 325, 351, 378, 406, 435, 465, 496, 528, 561, 595, 630, 666, 703,
            741, 780, 820, 861, 903, 946, 990, 1035, 1081, 1128, 1176, 1225, 1275, 1326, 1378,
            1431, 40_755, 48_024_900, 1_533_776_805,
        ];
        for val in truth {
            assert!(is_triangular(val));
        }
    }

    #[test]
    fn test_is_triangular_false() {
        let triangulars: HashSet<u64> = vec![
            0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210,
            231, 253, 276, 300, 325, 351, 378, 406, 435, 465, 496, 528, 561, 595, 630, 666, 703,
            741, 780, 820, 861, 903, 946, 990, 1035, 1081, 1128, 1176, 1225, 1275, 1326, 1378,
            1431,
        ].into_iter().collect();

        // Test ever number not in the set
        for i in (0..1431).filter(|x| !triangulars.contains(x)) {
            assert_eq!(is_triangular(i), false);
        }
    }

    #[test]
    fn test_is_pentagonal_true() {
        let truth = vec![
            1, 5, 12, 22, 35, 51, 70, 92, 117, 145, 176, 210, 247, 287, 330, 376, 425, 477, 532,
            590, 651, 715, 782, 852, 925, 1001, 1080, 1162, 1247, 1335, 1426, 1520, 1617, 1717,
            1820, 1926, 2035, 2147, 2262, 2380, 2501, 2625, 2752, 2882, 3015, 3151, 3290, 3432,
            3577, 3725, 3876, 4030, 4187, 40_755, 1_533_776_805,
        ];
        for val in truth {
            assert!(is_pentagonal(val));
        }
    }

    #[test]
    fn test_is_pentagonal_false() {
        let pentagonals: HashSet<u64> = vec![
            1, 5, 12, 22, 35, 51, 70, 92, 117, 145, 176, 210, 247, 287, 330, 376, 425, 477, 532,
            590, 651, 715, 782, 852, 925, 1001, 1080, 1162, 1247, 1335, 1426, 1520, 1617, 1717,
            1820, 1926, 2035, 2147, 2262, 2380, 2501, 2625, 2752, 2882, 3015, 3151, 3290, 3432,
            3577, 3725, 3876, 4030, 4187,
        ].into_iter().collect();

        // Test ever number not in the set
        for i in (0..4187).filter(|x| !pentagonals.contains(x)) {
            assert_eq!(is_pentagonal(i), false);
        }
    }

    #[test]
    fn test_is_hexagonal_true() {
        let truth = vec![
            1, 6, 15, 28, 45, 66, 91, 120, 153, 190, 231, 276, 325, 378, 435, 496, 561, 630, 703,
            780, 861, 946, 1035, 1128, 1225, 1326, 1431, 1540, 1653, 1770, 1891, 2016, 2145, 2278,
            2415, 2556, 2701, 2850, 3003, 3160, 3321, 3486, 3655, 3828, 4005, 4186, 4371, 4560,
            40_755, 1_533_776_805,
        ];
        for val in truth {
            assert!(is_hexagonal(val));
        }
    }

    #[test]
    fn test_is_hexagonal_false() {
        let hexagonals: HashSet<u64> = vec![
            1, 6, 15, 28, 45, 66, 91, 120, 153, 190, 231, 276, 325, 378, 435, 496, 561, 630, 703,
            780, 861, 946, 1035, 1128, 1225, 1326, 1431, 1540, 1653, 1770, 1891, 2016, 2145, 2278,
            2415, 2556, 2701, 2850, 3003, 3160, 3321, 3486, 3655, 3828, 4005, 4186, 4371, 4560,
        ].into_iter().collect();

        // Test ever number not in the set
        for i in (0..4560).filter(|x| !hexagonals.contains(x)) {
            assert_eq!(is_hexagonal(i), false);
        }
    }
}
