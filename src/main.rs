use std::env;
use std::fs;

mod error;
mod interpreter;
mod parser;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		eprintln!("Usage: {} [source file]", args[0]);
		return;
	}
	let source_file_path = &args[1];

	let source_code = fs::read_to_string(source_file_path).unwrap();

	let code = parser::parse(&source_code);

	let mut vm = interpreter::Interpreter::new(code, None);

	vm.run(|vm| {
		eprintln!(
			"#{: <4} - {: <?} => mp = {: <4?} *mp = {: <4?} memory = {:?}",
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
