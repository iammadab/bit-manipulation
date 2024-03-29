mod turn_off_least_significant_1_bit;
mod turn_off_least_significant_1_bit_and_turn_on_all_bits_after_it;
mod turn_off_trailing_1s;
mod turn_on_least_significant_0_bit;
mod turn_on_least_significant_0_bit_and_turn_off_all_bits_after_it;
mod turn_on_trailing_0s;

/// Return the binary string representation of a u32 number
fn bit_rep(value: u32) -> String {
    format!("{:b}", value)
}

/// Return the u32 representation of a binary string
fn u32_bit(bit_string: &str) -> u32 {
    u32::from_str_radix(&bit_string, 2).unwrap()
}
