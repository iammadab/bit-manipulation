#[cfg(test)]
mod tests {
    use crate::u32_bit;

    #[test]
    fn test_statement() {
        assert_eq!(
            u32_bit("1100") - 1,
            u32_bit("1011")
        );
    }
}