use super::regs::Regs;
use crate::r650x::alu::ALU;
use crate::r650x::pipeline::Pipeline;
use crate::microvm::memory::sparse::SparseAddressSpace;
use crate::microvm::memory::address_space::AddressSpace;
use crate::microvm::memory::MemoryError;
use crate::r650x::flags::{PSRFlag, FlagRegister};

pub struct Core {
    pipeline: Pipeline,
    regs: Regs,
    space: SparseAddressSpace<u16>
}

impl Core {
    fn cycle(&mut self) {

    }
    fn stack_push(&mut self, byte: u8) -> Result<(), MemoryError> {
        self.space.write_byte(self.sp_address(), byte)?;
        self.regs.sp -= 1;
        Ok(())
    }
    fn sp_address(&self) -> u16 {
        0x100 + (self.regs.sp as u16)
    }
    fn pc_high(&self) -> u8 {
        ((self.regs.pc & 0x00FF) >> 8) as u8
    }
    fn pc_low(&self) -> u8 {
        (self.regs.pc & 0xFF) as u8
    }
    fn push_pc(&mut self) -> Result<(), MemoryError>{
        self.stack_push(self.pc_high())?;
        self.stack_push(self.pc_low())
    }
    fn do_instruction(&mut self, instruction: u8) {

    }
    fn fetch(&mut self) -> Result<u8, MemoryError> {
        let b =self.space.read_byte(self.regs.pc)?;
        self.regs.pc += 1;
        Ok(b)
    }
    fn check_for_flags(&mut self, reg: u8) {
        if (reg as i8) < 0 {
            self.regs.psr.set(PSRFlag::Negative)
        }
        if reg == 0 {
            self.regs.psr.set(PSRFlag::Zero)
        }
    }
    fn m(&self) -> u8 {
        self.pipeline.m()
    }
    //Flags: CZVN
    fn i_adc(&mut self) {
        let carry = self.regs.psr.get(PSRFlag::Carry) as u8;
        let accumulator = self.regs.accumulator;
        let m = self.m();
        //Unsigned check for carry
        let inter_result = accumulator.overflowing_add(m);
        let result = (inter_result.0).overflowing_add(carry);
        let did_carry = (result.1) || (inter_result.1);

        //Signed check for overflow
        let inter_resulti = (accumulator as i8).overflowing_add(m as i8);
        let resulti = (inter_result.0).overflowing_add(carry as i8);
        let did_overflow = (resulti.1) || (inter_resulti.1);

        //Done so store the results
        self.regs.accumulator = result;
        if did_overflow {
            self.regs.psr.set(PSRFlag::Overflow)
        }
        if did_carry {
            self.regs.psr.set(PSRFlag::Carry)
        }
        self.check_for_flags(result);
    }
    fn i_and(&mut self) {
        let result = self.regs.accumulator & self.pipeline.m();
        self.regs.accumulator = result;
        self.check_for_flags(result);
    }
    fn i_asl(&mut self) {
        let result = self.pipeline.m().overflowing_shl(1);
        if result.1 {
            self.regs.psr.set(PSRFlag::Carry);
        }
        self.pipeline.latch_memory_value(result.0);
        self.check_for_flags(result.0);
    }
    fn i_bcc(&mut self) {
        if self.regs.psr.get(PSRFlag::Carry) == false {
            self.regs.pc = ((self.regs.pc as i16) + ((self.pipeline.m() as i8) as i16)) as u16; //Relative
        }
    }
    fn i_bcs(&mut self) {
        if self.regs.psr.get(PSRFlag::Carry) == true {
            self.regs.pc = ((self.regs.pc as i16) + ((self.pipeline.m() as i8) as i16)) as u16; //Relative
        }
    }
    fn i_beq(&mut self) {
        if self.regs.psr.get(PSRFlag::Zero) == true {
            self.regs.pc = ((self.regs.pc as i16) + ((self.pipeline.m() as i8) as i16)) as u16; //Relative
        }
    }
    fn i_tax(&mut self) {
        self.regs.x = self.regs.accumulator
    }
}