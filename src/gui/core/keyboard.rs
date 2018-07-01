pub struct KeyEvent {
	key_char: char
}

impl KeyEvent {
	pub fn of_char(key_char: char) -> KeyEvent {
		return KeyEvent { key_char: key_char }
	}
}
