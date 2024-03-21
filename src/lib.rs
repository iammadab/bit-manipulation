mod set_least_significant_1_bit_to_0;
mod set_least_significant_1_bit_to_0_and_set_rest_least_significant_bits_to_1s;

/// Return the binary string representation of a u32 number
fn bit_rep(value: u32) -> String {
    format!("{:b}", value)
}

/// Return the u32 representation of a binary string
fn u32_bit(bit_string: &str) -> u32 {
    u32::from_str_radix(&bit_string, 2).unwrap()
}