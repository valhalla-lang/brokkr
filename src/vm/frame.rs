use std::collections::HashSet;
use std::fmt;

use num_traits::{sign::Unsigned, cast::NumCast};
use super::address::Address;

#[derive(Clone, Copy, PartialEq)]
pub struct Instruction(pub usize);

/// Frame on the call-stack
#[derive(Debug, Clone)]
pub struct Frame {
    /// Path for non-compiled source file.
    pub source_file : String,
    /// Name for the frame, i.e. the module name.
    pub name : String,
    /// Vector of all constants used in the _function_.
    pub constants : Vec<Address>,
    /// Set of names for local variables.
    pub locals : HashSet<String>,
    /// Instructions for execution of the _function_.
    pub instructions : Vec<Instruction>,

    /// Maximum depth for the evaluation-stack.
    pub stack_depth : u16,
    /// Evaluation-stack (since it is a stack based VM).
    pub evaluations : Vec<Address>
}

impl<N> From<N> for Instruction where N: Unsigned + NumCast {
    fn from(other : N) -> Self {
        Instruction(num_traits::cast::<N, usize>(other).unwrap())
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}
