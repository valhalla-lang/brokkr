use std::collections::HashSet;
use crate::VoidPtr;

/// Frame on the call-stack
#[derive(Debug)]
pub struct Frame {
    /// Path for non-compiled source file.
    pub source_file : String,
    /// Name for the frame, i.e. the module name.
    pub name : String,
    /// Vector of all constants used in the _function_.
    pub constants : Vec<VoidPtr>,
    /// Set of names for local variables.
    pub locals : HashSet<String>,
    /// Instructions for execution of the _function_.
    pub instructions : Vec<usize>,

    /// Maximum depth for the evaluation-stack.
    pub stack_depth : u16,
    /// Evaluation-stack (since it is a stack based VM).
    pub evaluations : Vec<VoidPtr>
}

