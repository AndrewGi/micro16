use crate::μcompile::instructions::{Reg, OpLocationCode, Opcode};
use std::collections::HashMap;
use std::slice::Iter;
use std::collections::vec_deque::VecDeque;

#[derive(Eq, PartialEq, Clone)]
pub enum Arg {
	Constant(u16),
	RawReg(Reg),
	LoadReg(Reg),
	None
}
impl ToString for Arg {
	fn to_string(&self) -> String {
		match self {
			Arg::Constant(&c) => c.to_string(),
			Arg::RawReg(r) => "r" + r.to_string(),
			Arg::LoadReg(r) => "l" + r.to_string(),
			Arg::None => "_".to_string()
		}
	}
}
pub struct ExecutableLine {
	opcode: Opcode,
	output: Reg,
	arg1: Arg,
	arg2: Arg,
}
pub enum AssemblerLine {
	Executable(ExecutableLine),
	Label(String),
	Goto(String)
}
impl ToString for AssemblerLine {
	fn to_string(&self) -> String {
		self.opcode.to_string() + " " + self.output.to_string() + " "
			+ self.arg1.to_string() + " " + self.arg2.to_string()
	}
}
pub fn detect_arg_type(arg1: &Arg, arg2: &Arg) -> Option<OpLocationCode> {
	Some(match (arg1, arg2) {
		(Arg::Constant(_), Arg::Constant(_)) => OpLocationCode::TwoConstants,
		(Arg::RawReg(_), Arg::RawReg(_)) => OpLocationCode::TwoRawRegs,
		(Arg::LoadReg(_), Arg::LoadReg(_)) => OpLocationCode::TwoLoadRegs,
		(Arg::LoadReg(_), Arg::RawReg(_)) => OpLocationCode::LoadRegAndRawReg,
		(Arg::RawReg(_), Arg::LoadReg(_)) => OpLocationCode::RawRegAndLoadReg,
		(Arg::RawReg(_), Arg::Constant(_)) => OpLocationCode::RawRegAndConstant,
		(Arg::Constant(_), Arg::RawReg(_)) => OpLocationCode::ConstantAndRawReg,
		(Arg::None, Arg::None) => OpLocationCode::NoLocation,
		_ => return None
	})
}
#[derive(Default)]
struct Assembler<RawOut: std::io::Write> {
	lines: VecDeque<AssemblerLine>,
	labels: HashMap<String, u16>,
	unknown_labels: Vec<(String, u16)>,
	out: RawOut
}
impl<RawOut: std::io::Write> Assembler<RawOut> {
	fn next_line(&mut self) -> Option<AssemblerLine> {
		self.lines.pop_front()
	}
	fn add_label(&mut self, label: String) -> Option<()> {

	}
	fn process_executable(&mut self, executeable: ExecutableLine) -> Option<()> {

	}
	fn process_goto(&mut self, label: String) -> Option<()> {

	}

	pub fn process_line(&mut self) -> Option<()> {
		match self.current_line()? {
			AssemblerLine::Label(label) => self.add_label(label),
			AssemblerLine::Executable(executable) => self.process_executable(executable),
			AssemblerLine::Goto(label) => self.process_goto(label)
		}
	}
}