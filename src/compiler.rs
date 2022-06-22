use std::io;
use std::io::Write;

use crate::parser::Token;

pub mod constants {
	// https://wiki.osdev.org/CPU_Register_x86

	/// Data pointer register, any scratch register will do.
	/// But this register can **NOT** be clobbered inside the function.
	pub const DP: &'static str = "r10";

	/// Register used to return values
	pub const RET_REG: &'static str = "rax";

	/// 1st function parameter, memory start.
	///
	/// See [System V AMD64 ABI](https://wiki.osdev.org/System_V_ABI#x86-64).
	pub const P1_REG: &'static str = "rdi";

	/// Scratch register used for byte manipulation, any scratch register is fine, _except_ DP
	pub const TMP_REG: &'static str = "al"; // this is the 8 low bits of rax
}

use constants::*;

#[derive(Debug)]
pub struct Compiler {
	code: Vec<Token>,
	symbol_name: String,
}

impl Compiler {
	pub fn new(code: Vec<Token>, symbol_name: String) -> Self {
		Self { code, symbol_name }
	}

	pub fn compile(&self, writer: &mut dyn Write) -> Result<(), io::Error> {
		let mut loop_counter: u16 = 0;

		writeln!(writer, "global {}", self.symbol_name)?;
		writeln!(writer, "section .text")?;
		writeln!(writer, "{}:", self.symbol_name)?;

		// Initialize the memory pointer with the start of our memory
		writeln!(writer, "mov {}, {}", DP, P1_REG)?;

		for token in &self.code {
			use Token::*;
			let asm = match *token {
				MoveLeft => format!("sub {}, 0x1", DP),
				MoveRight => format!("add {}, 0x1", DP),
				IncreaseValue => format!("mov {0}, [{1}]\nadd {0}, 1\nmov [{1}], {0}", TMP_REG, DP),
				DecreaseValue => format!("mov {0}, [{1}]\nsub {0}, 1\nmov [{1}], {0}", TMP_REG, DP),
				Loop => {
					loop_counter += 1;
					format!(".loop_{}:", loop_counter)
				}
				JumpBack => {
					loop_counter -= 1;
					format!(
						"mov {0}, [{1}]\ntest {0}, {0}\njnz .loop_{2}",
						TMP_REG,
						DP,
						loop_counter + 1
					)
				}
				e => {
					panic!("{:?} is not supported", e);
				}
			};

			writeln!(writer, "{}", asm)?;
		}

		writeln!(writer, "mov {}, {}", RET_REG, DP)?;
		writeln!(writer, "ret")?;

		Ok(())
	}
}
