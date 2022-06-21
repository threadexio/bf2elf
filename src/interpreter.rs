use crate::error::Error;
use crate::parser::{types::*, Token};

use std::io::Bytes;
use std::io::{stdin, Stdin};
use std::io::{stdout, Stdout};
use std::io::{Read, Write};

const STACK_INITIAL_SIZE: usize = 32;

#[derive(Debug)]
pub struct Interpreter {
	/// Memory
	pub memory: Vec<Cell>,

	/// Program code
	pub code: Vec<Token>,

	/// Memory pointer
	pub mp: Address,

	/// Instruction pointer
	pub ip: Address,

	loop_stack: Vec<Address>,

	input: Bytes<Stdin>,
	output: Stdout,
}

#[allow(dead_code)]
impl Interpreter {
	pub fn new(code: Vec<Token>, memory_size: Option<Address>) -> Self {
		let mut memory: Vec<Cell> = Vec::new();
		memory.resize(memory_size.unwrap_or(30_000) as usize, 0);

		Self {
			memory,
			code,

			mp: 0,
			ip: 0,

			loop_stack: Vec::with_capacity(STACK_INITIAL_SIZE),

			input: stdin().bytes(),
			output: stdout(),
		}
	}

	pub fn get_memory(&self, address: Address) -> Option<&Cell> {
		self.memory.get(address as usize)
	}

	pub fn get_memory_mut(&mut self, address: Address) -> Option<&mut Cell> {
		self.memory.get_mut(address as usize)
	}

	pub fn get_current_memory(&self) -> &Cell {
		self.memory.get(self.mp as usize).unwrap()
	}

	pub fn get_current_memory_mut(&mut self) -> &mut Cell {
		self.memory.get_mut(self.mp as usize).unwrap()
	}

	pub fn evaluate(&mut self, code: Token) -> Result<(), Error> {
		use Token::*;

		match code {
			MoveLeft => {
				if self.mp <= 0 {
					Err(Error::OutsideBounds(self.ip))
				} else {
					self.mp -= 1;
					self.ip += 1;
					Ok(())
				}
			}
			MoveRight => {
				if self.mp >= (self.memory.len() - 1) as Address {
					Err(Error::OutsideBounds(self.ip))
				} else {
					self.mp += 1;
					self.ip += 1;
					Ok(())
				}
			}
			IncreaseValue => {
				let mem = self.get_current_memory_mut();
				*mem = mem.overflowing_add(1).0;
				self.ip += 1;
				Ok(())
			}
			DecreaseValue => {
				let mem = self.get_current_memory_mut();
				*mem = mem.overflowing_sub(1).0;
				self.ip += 1;
				Ok(())
			}
			Loop => {
				if *self.get_current_memory() != 0 {
					self.loop_stack.push(self.ip + 1);
				}
				self.ip += 1;
				Ok(())
			}
			JumpBack => {
				if self.loop_stack.len() == 0 {
					return Err(Error::ExpectedLoopStart(self.ip));
				} else {
					if *self.get_current_memory() != 0 {
						self.ip = *self.loop_stack.last().unwrap();
					} else {
						let _ = self.loop_stack.pop();
						self.ip += 1;
					}
				}

				Ok(())
			}
			GetInput => match self.input.next().unwrap() {
				Err(e) => Err(Error::Io(e)),
				Ok(v) => {
					*self.get_current_memory_mut() = v;
					self.ip += 1;
					Ok(())
				}
			},
			Print => match write!(self.output, "{}", *self.get_current_memory() as char) {
				Err(e) => Err(Error::Io(e)),
				Ok(_) => {
					let _ = self.output.flush();
					self.ip += 1;
					Ok(())
				}
			},
		}
	}

	pub fn step(&mut self) -> Result<(), Error> {
		self.evaluate(self.code[self.ip as usize])?;
		Ok(())
	}

	pub fn run<F>(&mut self, watchdog: F) -> Result<(), Error>
	where
		F: Fn(&mut Self) -> Result<(), ()>,
	{
		while (self.ip as usize) < self.code.len() {
			match watchdog(self) {
				Err(_) => return Err(Error::WatchdogError),
				Ok(_) => {}
			}

			self.step()?;
		}

		Ok(())
	}
}
