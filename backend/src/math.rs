pub const fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_works_1() { assert_eq!(4, add(2, 2)) }

    #[test]
    fn math_works_2() { assert_eq!(-4, add(-8, 3)) }
}
