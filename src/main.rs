use std::fs;

mod error;
mod parser;

mod compiler;
mod interpreter;

use std::io::BufWriter;

use clap::clap_derive::*;
use clap::Parser;

#[derive(Debug, ArgEnum, Clone)]
enum Operation {
	Compile,
	Interpret,
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
	#[clap(arg_enum, value_parser)]
	operation: Operation,

	#[clap(
		short = 'o',
		long = "outfile",
		help = "Output assembly file",
		default_value_t = String::from("out.asm")
	)]
	outfile: String,

	#[clap(value_parser)]
	infile: String,

	#[clap(short = 's', long = "symbol", value_parser, help = "Exported symbol name",default_value_t = String::from("test"))]
	symbol: String,
}

fn main() {
	let config = Config::parse();

	let source_code = fs::read_to_string(config.infile).unwrap();

	let code = parser::parse(&source_code);

	match config.operation {
		Operation::Compile => {
			let compiler = compiler::Compiler::new(code, config.symbol);
			let asm_file = fs::File::create(&config.outfile).unwrap();
			let mut writer = BufWriter::new(asm_file);

			compiler.compile(&mut writer).unwrap();
		}
		Operation::Interpret => {
			let mut vm = interpreter::Interpreter::new(code, None);

			vm.run(|vm| {
				eprintln!(
					"#{: <4} - {: <15?} => mp = {: <4?} *mp = {: <4?} memory = {:?}",
					vm.ip,
					&vm.code[vm.ip as usize],
					vm.mp,
					*vm.get_current_memory(),
					&vm.memory[..5]
				);

				Ok(())
			})
			.unwrap();
		}
	}
}

#[cfg(test)]
mod test {
	use crate::{error, interpreter, parser};

	#[test]
	fn test_move_left() {
		let code = parser::parse("<");
		let mut vm = interpreter::Interpreter::new(code, None);
		vm.mp = 1;
		vm.step().unwrap();
		assert_eq!(vm.mp, 0);
	}

	#[test]
	fn test_move_out_of_bounds_left() {
		let code = parser::parse("<");
		let mut vm = interpreter::Interpreter::new(code, None);
		assert_eq!(
			vm.run(|_| Ok(())).unwrap_err(),
			error::Error::OutsideBounds(0)
		);
	}

	#[test]
	fn test_move_right() {
		let code = parser::parse(">");
		let mut vm = interpreter::Interpreter::new(code, None);
		vm.step().unwrap();
		assert_eq!(vm.mp, 1);
	}

	#[test]
	fn test_move_out_of_bounds_right() {
		let code = parser::parse(">");
		let mut vm = interpreter::Interpreter::new(code, Some(1));
		assert_eq!(
			vm.run(|_| Ok(())).unwrap_err(),
			error::Error::OutsideBounds(0)
		);
	}

	#[test]
	fn test_add_overflow() {
		let code = parser::parse("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
		let mut vm = interpreter::Interpreter::new(code, None);
		vm.run(|_| Ok(())).unwrap();
		assert_eq!(*vm.get_memory(0).unwrap(), 1_u8);
	}

	#[test]
	fn test_sub_overflow() {
		let code = parser::parse("-");
		let mut vm = interpreter::Interpreter::new(code, None);
		vm.run(|_| Ok(())).unwrap();
		assert_eq!(*vm.get_memory(0).unwrap(), 255_u8);
	}

	#[test]
	fn test_loop() {
		let code = parser::parse("+++++[-]+");
		let mut vm = interpreter::Interpreter::new(code, None);
		vm.run(|_| Ok(())).unwrap();
		assert_eq!(*vm.get_memory(0).unwrap(), 1_u8);
	}

	#[test]
	fn test_loop_no_start() {
		let code = parser::parse("+++++-]");
		let mut vm = interpreter::Interpreter::new(code, None);
		assert_eq!(
			vm.run(|_| Ok(())).unwrap_err(),
			error::Error::ExpectedLoopStart(6)
		);
	}
}
