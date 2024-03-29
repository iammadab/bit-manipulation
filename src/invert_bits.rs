#[cfg(test)]
mod tests {
    use crate::u32_bit;

    #[test]
    fn test_statement() {
        let v = u32_bit("101110");
        assert_eq!(!v, u32_bit("11111111111111111111111111010001"));
    }
}
