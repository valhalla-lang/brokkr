use num_derive::FromPrimitive;

#[repr(usize)]
#[allow(non_camel_case_types)]
#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq)]
pub enum Operators {
    EOI         = 0,   // TAKES 0 OPERAND(s) (Not a proper operator)

    PUSH_CONST  = 1,   // TAKES 1 OPERAND(s)
    PUSH_LOCAL  = 2,   // TAKES 1 OPERAND(s)
    PUSH_SUPER  = 3,   // TAKES 1 OPERAND(s)
    POP         = 4,   // TAKES 0 OPERAND(s)
    STORE_LOCAL = 5,   // TAKES 1 OPERAND(s)
    DUP         = 6,   // TAKES 0 OPERAND(s)
    DUP_N       = 7,   // TAKES 1 OPERAND(s)
    SWAP        = 8,   // TAKES 0 OPERAND(s)
    CALL_1      = 9,   // TAKES 0 OPERAND(s)
    CHECK_TYPE  = 10,  // TAKES 0 OPERAND(s)
    CAST        = 11,  // TAKES 2 OPERAND(s) (2 operands, 1 out of 2 bytes for each)
    MAKE_FUNC   = 12,  // TAKES 0 OPERAND(s)
    YIELD       = 13,  // TAKES 0 OPERAND(s)
    RAW_PRINT   = 14,  // TAKES 1 OPERAND(s)

    N_ADD       = 40,  // TAKES 0 OPERAND(s)
    I_ADD       = 41,  // TAKES 0 OPERAND(s)
    R_ADD       = 42,  // TAKES 0 OPERAND(s)
    U_ADD       = 43,  // TAKES 0 OPERAND(s)
    CONCAT      = 44,  // TAKES 0 OPERAND(s)
    N_SUB       = 45,  // TAKES 0 OPERAND(s)
    I_SUB       = 46,  // TAKES 0 OPERAND(s)
    R_SUB       = 47,  // TAKES 0 OPERAND(s)
    U_SUB       = 48,  // TAKES 0 OPERAND(s)
    N_MUL       = 49,  // TAKES 0 OPERAND(s)
    I_MUL       = 50,  // TAKES 0 OPERAND(s)
    R_MUL       = 51,  // TAKES 0 OPERAND(s)
    U_MUL       = 52,  // TAKES 0 OPERAND(s)
    N_DIV       = 53,  // TAKES 0 OPERAND(s)
    I_DIV       = 54,  // TAKES 0 OPERAND(s)
    R_DIV       = 55,  // TAKES 0 OPERAND(s)
    U_DIV       = 56,  // TAKES 0 OPERAND(s)

    HALT        = 200, // TAKES 1 OPERAND(s)

    // Misc- / Meta-codes
    SET_LINE = 254,  // TAKES 1 OPERAND(s)
    NOP = 255,       // TAKES 0 OPERAND(s)
}

impl Operators {
    #[must_use]
    pub fn takes_operand(self) -> bool {
        match self {
            Self::HALT
            | Self::PUSH_CONST
            | Self::PUSH_LOCAL
            | Self::PUSH_SUPER
            | Self::STORE_LOCAL
            | Self::DUP_N
            | Self::CAST
            | Self::RAW_PRINT
            | Self::SET_LINE => true,
            _ => false
        }
    }
}
