# Brokkr Bytecode Specification

Sizes are as follows:

| Type     | Size                     |
|----------|--------------------------|
| Operator | 1 Byte  (8  Bits,  `u8`)  |
| Operand  | 2 Bytes (16 Bits, `u16`) |

## Bytecodes
| Byte | Name | Operands | Description |
|:---:|:---:|---|---|
| `0x00`<br />(0) | `EOI` | <center>N/A</center> | End Of Instructions! This is not a real opcode and only appears at the end of the instruction byte-stream to signal that there are no more instructions to be read. Essentially it's just a null-terminator. |
| `0xc8`<br />(200) | `HALT` | 1 — Exit Code (Integer) | Stops execution. |
| `0x01`<br />(1) | `PUSH_CONST` | 1 — Unsigned index to constants table | Pushes a constant onto the stack. Constant is specified by a given index to the constant-table, containing all local constant. |
| `0x02`<br />(2) | `PUSH_LOCAL` | 1 — Local variable index (Unsigned) | Pushes a local variable (e.g. a function param) onto the stack. The local variable is identified by an index operand, to where it is in the current symbol table. |
| `0x03`<br />(3) | `PUSH_SUPER` | 1 — Name of super-scoped variable (String) | When a variable is out of the local scope, load it onto the stack by searching up the super-scope by its name, (much slower than `PUSH_LOCAL`). |
| `0x04`<br />(4) | `POP` | <center>N/A</center> | Pops the top element off of the stack. |
| `0x05`<br />(5) | `STORE_LOCAL` | 1 — Index to store local variable in table (Unsigned) | Pops the top value and stores it in the local symbol table at a given index. |
| `0x06`<br />(6) | `DUP` | <center>N/A</center> | Duplicates what was on the top of the stack, by popping it off and then pushing two copies. |
| `0x07`<br />(7) | `DUP_N` | 1 — The number of duplicates | Duplicates the top value on the stack N times. |
| `0x08`<br />(8) | `SWAP` | <center>N/A</center> | Swaps the position of the top two stack elements. |
| `0x09`<br />(9) | `CALL_1` | <center>N/A</center> | Calls function at top of stack with argument at second from top of stack. |
| `0x0a`<br />(10) | `CHECK_TYPE` | <center>N/A</center> | Checks if second from top of stack value is a member of the set/type loaded from the top of the stack. |
| `0x0b`<br />(11) | `CAST` | 1 — One byte, specifies the origin type.<br />2 — One byte, specifies the destination type. | Changes the top-of-stack value to have a different primitive type. |
| `0x0c`<br />(12) | `MAKE_FUNC` | <center>N/A</center> | Takes the top-of-stack value, referencing a code block, and makes it a function, with all the special properties that come with that. |
| `0x0d`<br />(13) | `YIELD` | <center>N/A</center> | Causes the current function to yield a value (i.e. return). |
| `0x0e`<br />(14) | `RAW_PRINT` | <center>N/A</center> | Native debug print of `TOS`. Should not be used unless working on the compiler. |
| `0x28`<br />(40) | `N_ADD` | <center>N/A</center> | Addition between two pointer-sized unsigned integers. Pops top two elements from the stacks and adds them together, pushing the result. |
| `0x29`<br />(41) | `I_ADD` | <center>N/A</center> | Addition between two pointer-sized signed integers. Pops top two elements from the stacks and adds them together, pushing the result. |
| `0x2a`<br />(42) | `R_ADD` | <center>N/A</center> | Addition between two 64-bit floating-point numbers. Pops top two elements from the stacks and adds them together, pushing the result. |
| `0x2b`<br />(43) | `U_ADD` | <center>N/A</center> | Addition between two values of unknown types, found out at runtime. Pops top two elements from the stacks and adds them together, pushing the result. |
| `0x2c`<br />(44) | `CONCAT` | <center>N/A</center> | Works like add, but concatenates strings. |
| `0x2d`<br />(45) | `N_SUB` | <center>N/A</center> | Subtraction between two pointer-sized unsigned integers. Pops top two elements from the stacks and subtracts them together, pushing the result. |
| `0x2e`<br />(46) | `I_SUB` | <center>N/A</center> | Subtraction between two pointer-sized signed integers. Pops top two elements from the stacks and subtracts them together, pushing the result. |
| `0x2f`<br />(47) | `R_SUB` | <center>N/A</center> | Subtraction between two 64-bit floating-point numbers. Pops top two elements from the stacks and subtracts them together, pushing the result. |
| `0x30`<br />(48) | `U_SUB` | <center>N/A</center> | Subtraction between two values of unknown types, found out at runtime. Pops top two elements from the stacks and subtracts one from the other, pushing the result. |
| `0x31`<br />(49) | `N_MUL` | <center>N/A</center> | Multiplication between two pointer-sized unsigned integers. Pops top two elements from the stacks and multiplies them together, pushing the result. |
| `0x32`<br />(50) | `I_MUL` | <center>N/A</center> | Multiplication between two pointer-sized signed integers. Pops top two elements from the stacks and multiplies them together, pushing the result. |
| `0x33`<br />(51) | `R_MUL` | <center>N/A</center> | Multiplication between two 64-bit floating-point numbers. Pops top two elements from the stacks and multiplies them together, pushing the result. |
| `0x34`<br />(52) | `U_MUL` | <center>N/A</center> | Multiplication between two values of unknown types, found out at runtime. Pops top two elements from the stacks and multiplies them together, pushing the result. |
| `0x35`<br />(53) | `N_DIV` | <center>N/A</center> | Division between two pointer-sized unsigned integers. Pops top two elements from the stacks and finds their quotient, pushing the result. |
| `0x36`<br />(54) | `I_DIV` | <center>N/A</center> | Division between two pointer-sized signed integers. Pops top two elements from the stacks and finds their quotient, pushing the result. |
| `0x37`<br />(55) | `R_DIV` | <center>N/A</center> | Division between two 64-bit floating-point numbers. Pops top two elements from the stacks and finds their quotient, pushing the result. |
| `0x38`<br />(56) | `U_DIV` | <center>N/A</center> | Division between two values of unknown types, found out at runtime. Pops top two elements from the stacks and finds their quotient, pushing the result. |
| `0xfe`<br />(254) | `SET_LINE` | 1 — Current line number, given directly as `u16`. Operand value is line number. | Sets the current line number that the subsequent bytecode instructions correspond to in the code source file. |
| `0xff`<br />(255) | `NOP` | <center>N/A</center> | <u>N</u>o <u>Op</u>eration. Does nothing. You shouldn't see this in the final compiled bytecode, it may only exists temporarily while the bytecode is being produced. |
