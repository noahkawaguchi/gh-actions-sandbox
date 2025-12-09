use rand::Rng;

pub const fn add(x: i32, y: i32) -> i32 { x + y }

/// Generates a random number in the range [0, max].
pub fn random_number_le(max: u8) -> u8 { rand::rng().random_range(0..=max) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_works_1() { assert_eq!(4, add(2, 2)) }

    #[test]
    fn math_works_2() { assert_eq!(-5, add(-8, 3)) }

    #[test]
    fn rand_works() {
        let max = 88;
        let num = random_number_le(max);
        assert!(num <= max);
    }
}
