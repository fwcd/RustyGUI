pub fn clamp_to_unsigned(value: i32) -> u32 {
	if value <= 0 { 0u32 } else { (value as u32) }
}
