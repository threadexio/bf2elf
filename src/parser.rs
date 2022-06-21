pub mod types {
	pub type Cell = u8;
	pub type Address = u64;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Token {
	MoveLeft,
	MoveRight,
	IncreaseValue,
	DecreaseValue,
	Loop,
	JumpBack,
	GetInput,
	Print,
}

pub fn parse(source_code: &str) -> Vec<Token> {
	let mut tokens: Vec<Token> = Vec::with_capacity(source_code.len());

	use Token::*;
	for symbol in source_code.chars() {
		match symbol {
			'<' => tokens.push(MoveLeft),
			'>' => tokens.push(MoveRight),
			'+' => tokens.push(IncreaseValue),
			'-' => tokens.push(DecreaseValue),
			'.' => tokens.push(Print),
			',' => tokens.push(GetInput),
			'[' => tokens.push(Loop),
			']' => tokens.push(JumpBack),
			_ => {}
		}
	}

	tokens
}
