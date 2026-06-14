pub fn try_add(x: i32, y: i32) -> Result<i32, String> {
    x.checked_add(y)
        .ok_or_else(|| format!("Overflowed i32 adding {x} + {y}"))
}

/// Generates a random number in the range [0, max].
pub fn random_number_le(max: u8) -> u8 { rand::random_range(0..=max) }

#[cfg(test)]
mod tests {
    use {super::*, std::assert_matches};

    #[test]
    fn math_works_1() { assert_eq!(try_add(2, 2), Ok(4)) }

    #[test]
    fn math_works_2() { assert_eq!(try_add(-8, 3), Ok(-5)) }

    #[test]
    fn overflow_detected() { assert_matches!(try_add(i32::MAX, 1), Err(_)) }

    #[test]
    fn underflow_detected() { assert_matches!(try_add(i32::MIN, -1), Err(_)) }

    #[test]
    fn rand_works() {
        let max = 88;
        let num = random_number_le(max);
        assert!(num <= max);
    }
}
