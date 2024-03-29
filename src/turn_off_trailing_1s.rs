#[cfg(test)]
mod tests {
    use crate::u32_bit;

    #[test]
    fn test_statement() {
        let v = u32_bit("1010111");
        assert_eq!(
            v & (v + 1),
            u32_bit("1010000")
        );
    }
}
