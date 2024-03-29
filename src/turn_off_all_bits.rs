#[cfg(test)]
mod tests {
    use crate::u32_bit;

    #[test]
    fn test_statement() {
        let v = u32_bit("101010010");
        assert_eq!(!v & v, 0);
    }
}
