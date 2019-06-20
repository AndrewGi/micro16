use crate::microcompile::instructions::{Arg, Reg, OpArgsTypes, Opcode};
use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;
type UInt = u16;
pub enum AssemblerArg {
	Arg(Arg),
	Variable(String)
}
#[derive(Clone)]
struct ExecutableLine {
	opcode: Opcode,
	output: Reg,
	arg1: Arg,
	arg2: Arg,
}
#[derive(Clone)]
pub enum AssemblerLine {
	Executable(ExecutableLine),
	Label(String),
	NewVar{name: String, var_type: String, data: String}
}
impl ToString for ExecutableLine {
	fn to_string(&self) -> String {
		self.opcode.to_string() + " " + self.output.to_string().as_str() + " "
			+ self.arg1.to_string().as_str() + " " + self.arg2.to_string().as_str()
	}
}
pub fn detect_arg_type(arg1: &Arg, arg2: &Arg) -> Option<OpArgsTypes> {
	Some(match (arg1, arg2) {
		(Arg::Constant(_), Arg::Constant(_)) => OpArgsTypes::TwoConstants,
		(Arg::RawReg(_), Arg::RawReg(_)) => OpArgsTypes::TwoRawRegs,
		(Arg::LoadReg(_), Arg::LoadReg(_)) => OpArgsTypes::TwoLoadRegs,
		(Arg::LoadReg(_), Arg::RawReg(_)) => OpArgsTypes::LoadRegAndRawReg,
		(Arg::RawReg(_), Arg::LoadReg(_)) => OpArgsTypes::RawRegAndLoadReg,
		(Arg::RawReg(_), Arg::Constant(_)) => OpArgsTypes::RawRegAndConstant,
		(Arg::Constant(_), Arg::RawReg(_)) => OpArgsTypes::ConstantAndRawReg,
		(Arg::None, Arg::None) => OpArgsTypes::NoLocation,
		_ => return None
	})
}
#[derive(Clone)]
pub enum VarType {
	I8,
	U8,
	U16,
	I16,
	Array(Box<VarType>, UInt),
	Label
}
impl VarType {
	pub fn size(&self) -> UInt {
		match self {
			VarType::I8 => 1,
			VarType::U8 => 1,
			VarType::U16 => 2,
			VarType::I16 => 2,
			VarType::Array(t, amount) => {t.as_ref().size() * *amount}, //todo: could overflow
			VarType::Label => 0,
		}
	}
}
#[derive(Clone)]
struct VarDeclaration {
	name: String,
	var_type: VarType,
	address: UInt
}
#[derive(Default)]
struct Assembler {
	lines: VecDeque<AssemblerLine>,
	vars: HashMap<String, VarDeclaration>,
	unknown_addresses: Vec<(String, UInt)>,
	pc_offset: UInt,
	out: Vec<u8>
}
enum AssemblerError {
	LabelAlreadyExists,
	VariableDoesntExists,
	PCOverflow
}
impl Assembler {
	pub fn new(pc_offset: UInt) -> Assembler {
		Assembler {
			pc_offset,
			..Default::default()
		}
	}
	pub fn pc(&self) -> UInt {
		self.pc_offset + self.out.len() as UInt
	}
	fn next_line(&mut self) -> Option<AssemblerLine> {
		self.lines.pop_front()
	}
	fn add_label(&mut self, label: String) -> Result<(), AssemblerError> {
		self.add_variable(label, VarType::Label)
	}
	fn add_variable(&mut self, var_name: String, var_type: VarType) -> Result<(), AssemblerError> {
		let var_dec = VarDeclaration {
			name: var_name.to_string(),
			var_type: var_type.clone(),
			address: self.allocate(var_type.size())?
		};
		use std::collections::hash_map::*;
		if let Entry::Vacant(e) = self.vars.entry(var_dec.name.clone()) {
			e.insert(var_dec.clone());
		} else {
			return Err(AssemblerError::LabelAlreadyExists);
		};
		for unknown in &self.unknown_addresses {
			if unknown.0 == var_dec.name {
				*crate::microcompile::transmute_slice_mut::<UInt>(&mut self.out[unknown.1 as usize..]) = var_dec.address;
			}
		}
		self.unknown_addresses.retain(|u| u.0 != var_dec.name);
		Ok(())
	}
	fn find_var(&self, var_name: &str) -> Option<&VarDeclaration> {
		self.vars.get(var_name)
	}
	fn process_executable(&mut self, executable: ExecutableLine) -> Result<(), AssemblerError> {
		Ok(())
	}
	fn add_new_var(&mut self, name: String, var_type: String, data: String) -> Result<(), AssemblerError> {
		unimplemented!()
	}
	fn allocate(&mut self, amount: UInt) -> Result<UInt, AssemblerError> {
		amount.checked_add(self.out.len() as UInt).ok_or(AssemblerError::PCOverflow)?;
		let pc = self.pc();
		if amount != 0 {
			self.out.resize(self.out.len() + amount as usize, 0);
		}
		Ok(pc)
	}
	fn process_line(&mut self, line: AssemblerLine) -> Result<(), AssemblerError> {
		match line {
			AssemblerLine::Label(label) => self.add_label(label),
			AssemblerLine::Executable(executable) => self.process_executable(executable),
			AssemblerLine::NewVar{name, var_type, data} => self.add_new_var(name,var_type,data)
		}
	}
	pub fn process_next(&mut self) -> Option<Result<(), AssemblerError>> {
		let line = self.next_line()?;
		Some(self.process_line(line))
	}
}
#[cfg(test)]
mod tests {
	#[cfg(test)]
	fn test1() {

	}
}