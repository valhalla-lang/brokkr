use super::address::Address;
use super::opcodes::Operators as Op;
use super::frame::{Frame, Instruction as Instr};
use super::call_stack::CallStack;

use num_traits::FromPrimitive;

use std::mem;

macro_rules! cast_from {
    ($from:expr, $tos:expr, $to:ty) => { unsafe {
        match $from {
            0x01 => Address::from_value(&($tos.0 as $to)),
            0x02 => Address::from_value(&($tos.value::<isize>() as $to)),
            0x03 => Address::from_value(&($tos.value::<f64>()   as $to)),
            _ => Address::null()
        }
    }};
}

/// Implement execution for frames.
impl Frame {
    #[inline]
    fn current_instr(&self) -> Instr { self.instructions[self.pc] }

    pub fn execute(&mut self) -> Address {
        self.pc = 0;
        loop {
            let top = self.evaluations.len().wrapping_sub(1);

            let instr = self.current_instr();
            if instr == Instr(0x00) { return Address::null(); }

            let instr_word = usize::from(instr);
            let op = Op::from_usize(instr_word)
                .expect("Operand was consumed as operator, this shouldn't happen.");

            if op == Op::YIELD {
                let tos = self.evaluations.get(top)
                    .expect("Cannot yield when stack is empty.");
                return *tos;
            }

            match op {
                Op::SET_LINE => {
                    self.pc += 1;
                    self.line = usize::from(self.instructions[self.pc]);
                },
                Op::PUSH_CONST => {
                    self.pc += 1;
                    let index = usize::from(self.current_instr());
                    self.evaluations.push(self.constants[index]);
                },
                Op::PUSH_LOCAL => {
                    self.pc += 1;
                    let index = usize::from(self.current_instr());
                    self.evaluations.push(self.locals[index]);
                },
                Op::POP => { self.evaluations.pop(); },
                Op::STORE_LOCAL => {
                    self.pc += 1;
                    let index = usize::from(self.current_instr());
                    let value = self.evaluations.pop()
                        .expect("Stack empty, cannot store value.");
                    self.locals[index] = value;
                },
                Op::CAST => {
                    self.pc += 1;
                    let operand = usize::from(self.current_instr());
                    let to = operand & 0x00ff;
                    let from = operand >> 8;

                    let tos = self.evaluations.get(top)
                        .expect("Cast, but stack is empty.");

                    self.evaluations[top] = match to {
                        // Real
                        0x03 => cast_from!(from, tos, f64),
                        // Integer
                        0x02 => cast_from!(from, tos, isize),
                        // Natural
                        0x01 => cast_from!(from, tos, usize),
                        _ => panic!("Unknown cast type (0x{:02X})", to)
                    };
                },
                Op::RAW_PRINT => {
                    let tos = self.evaluations[top];

                    self.pc += 1;
                    let operand = usize::from(self.current_instr());

                    print!("raw: ");
                    unsafe {
                        match operand {
                            0x01 => println!("{}", tos.value::<usize>()),
                            0x02 => println!("{}", tos.value::<isize>()),
                            0x03 => println!("{}", tos.value::<f64>()),
                            0x04 => println!("{}", tos.reference::<&str>()),
                            _ => panic!("Unhandled RAW_PRINT type.")
                        }
                    };
                },
                Op::R_ADD => {
                    let tos = self.evaluations.get(top)
                        .expect("Cannot perform addition when stack is empty.");
                    let sos = self.evaluations.get(top - 1)
                        .expect("Cannot perform addition when stack is only 1 deep.");

                    self.evaluations[top] = unsafe {
                        Address::from_value(&(
                            tos.value::<f64>() + sos.value::<f64>()
                        ))
                    }
                },
                _ => panic!(
                    "Unaccounted for opcode (0x{:04X}), cannot execute.",
                    instr_word)
            }

            self.pc += 1;
        }
    }
}

pub struct Environment {
    pub calls : CallStack
}

impl Environment {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn entry(&mut self, frame : Frame) {
        if !self.calls.stack.is_empty() {
            panic!("Cannot set entry point when stack is not empty.");
        }

        self.calls.push(frame);
    }

    #[inline]
    pub fn execute(&mut self) {
        let last = self.calls.stack.len() - 1;
        let top = &mut self.calls.stack[last];

        top.execute();
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            calls: CallStack::new(),
        }
    }
}
