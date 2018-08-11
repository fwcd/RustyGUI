#[derive(Copy, Clone)]
pub struct KeyEvent {
	key_char: char
}

impl KeyEvent {
	pub fn of_char(key_char: char) -> KeyEvent {
		return KeyEvent { key_char: key_char }
	}
	
	pub fn get_char(&self) -> char { self.key_char }
}
