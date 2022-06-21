use std::io;

use crate::parser::types::*;

#[derive(Debug)]
pub enum Error {
	OutsideBounds(Address),
	ExpectedLoopStart(Address),
	Io(io::Error),
	WatchdogError,
}

impl PartialEq for Error {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::OutsideBounds(l), Self::OutsideBounds(r)) => *l == *r,
			(Self::ExpectedLoopStart(l), Self::ExpectedLoopStart(r)) => *l == *r,
			(Self::Io(_), Self::Io(_)) => true,
			_ => false,
		}
	}
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use Error::*;
		write!(
			f,
			"{}",
			match self {
				OutsideBounds(pos) =>
					format!("pointer went outside of bounds (instruction {})", pos),
				ExpectedLoopStart(pos) => format!("missing loop start (instruction {})", pos),
				Io(e) => format!("io error: {}", e),
				WatchdogError => format!("watchdog stopped execution"),
			}
		)
	}
}

impl std::error::Error for Error {}
