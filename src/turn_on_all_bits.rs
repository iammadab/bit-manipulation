#[cfg(test)]
mod tests {
    use crate::u32_bit;

    #[test]
    fn test_statement() {
        let v = u32_bit("10110010");
        assert_eq!(v | !v, u32::MAX)
    }
}
