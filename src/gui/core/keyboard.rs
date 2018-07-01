pub struct KeyEvent {
	keyChar: char
}

impl KeyEvent {
	pub fn of_char(keyChar: char) -> KeyEvent {
		return KeyEvent { keyChar: keyChar }
	}
}
