# Brokkr Bytecode Specification

Sizes are as follows:

| Type     | Size                     |
|----------|--------------------------|
| Operator | 1 Byte  (8  Bits, `u8`)  |
| Operand  | 2 Bytes (16 Bits, `u16`) |

## Bytecodes
| Byte | Name | Operands | Description |
|:---:|:---:|---|---|
| `0xc8` | `HALT` | 1 — Exit Code (Integer) | Stops execution. |
| `0x01` | `PUSH_CONST` | 1 — Unsigned index to constants table | Pushes a constant onto the stack. Constant is specified by a given index to the constant-table, containing all local constant. |
| `0x02` | `PUSH_LOCAL` | 1 — Local variable index (Unsigned) | Pushes a local variable (e.g. a function param) onto the stack. The local variable is identified by an index operand, to where it is in the current symbol table. |
| `0x03` | `PUSH_SUPER` | 1 — Name of super-scoped variable (String) | When a variable is out of the local scope, load it onto the stack by searching up the super-scope by its name, (much slower than `PUSH_LOCAL`). |
| `0x04` | `POP` |  | Pops the top element off of the stack. |
| `0x05` | `STORE_LOCAL` | 1 — Index to store local variable in table (Unsigned) | Pops the top value and stores it in the local symbol table at a given index. |
| `0x06` | `STORE_SUPER` | 1 — Name of super-scoped variable (String) | When a variable is out of the local scope, pop it off of the stack to store it in the super-scope searching up the frames for its name, (much slower than `STORE_LOCAL`). |
| `0x06` | `DUP` |  | Duplicates what was on the top of the stack, by popping it off and then pushing two copies. |
| `0x07` | `DUP_N` | 1 — The number of duplicates | Duplicates the top value on the stack N times. |
| `0x08` | `SWAP` |  | Swaps the position of the top two stack elements. |
| `0x28` | `N_ADD` |  | Addition between two pointer-sized unsigned integers. Pops top two elements from the stacks and adds them together, pushing the result. |
| `0x29` | `I_ADD` |  | Addition between two pointer-sized signed integers. Pops top two elements from the stacks and adds them together, pushing the result. |
| `0x2a` | `R_ADD` |  | Addition between two 64-bit floating-point numbers. Pops top two elements from the stacks and adds them together, pushing the result. |
| `0x2b` | `U_ADD` |  | Addition between two values of unknown types, found out at runtime. Pops top two elements from the stacks and adds them together, pushing the result. |
| `0x2c` | `CONCAT` |  | Works like add, but concatenates strings. |
| `0x2d` | `N_SUB` |  | Subtraction between two pointer-sized unsigned integers. Pops top two elements from the stacks and subtracts them together, pushing the result. |
| `0x2e` | `I_SUB` |  | Subtraction between two pointer-sized signed integers. Pops top two elements from the stacks and subtracts them together, pushing the result. |
| `0x2f` | `R_SUB` |  | Subtraction between two 64-bit floating-point numbers. Pops top two elements from the stacks and subtracts them together, pushing the result. |
| `0x30` | `U_SUB` |  | Subtraction between two values of unknown types, found out at runtime. Pops top two elements from the stacks and subtracts one from the other, pushing the result. |
| `0x31` | `N_MUL` |  | Multiplication between two pointer-sized unsigned integers. Pops top two elements from the stacks and multiplies them together, pushing the result. |
| `0x32` | `I_MUL` |  | Multiplication between two pointer-sized signed integers. Pops top two elements from the stacks and multiplies them together, pushing the result. |
| `0x33` | `R_MUL` |  | Multiplication between two 64-bit floating-point numbers. Pops top two elements from the stacks and multiplies them together, pushing the result. |
| `0x34` | `U_MUL` |  | Multiplication between two values of unknown types, found out at runtime. Pops top two elements from the stacks and multiplies them together, pushing the result. |
| `0x35` | `N_DIV` |  | Division between two pointer-sized unsigned integers. Pops top two elements from the stacks and finds their quotient, pushing the result. |
| `0x36` | `I_DIV` |  | Division between two pointer-sized signed integers. Pops top two elements from the stacks and finds their quotient, pushing the result. |
| `0x37` | `R_DIV` |  | Division between two 64-bit floating-point numbers. Pops top two elements from the stacks and finds their quotient, pushing the result. |
| `0x38` | `U_DIV` |  | Division between two values of unknown types, found out at runtime. Pops top two elements from the stacks and finds their quotient, pushing the result. |
| `0xfe` | `SET_LINE` | 1 — Current line number, given directly as `u16`. Operand value is line number. | Sets the current line number that the subsequent bytecode instructions correspond to in the code source file. |
| `0xff` | `NOP` |  | <u>N</u>o <u>Op</u>eration. Does nothing. You shouldn't see this in the final compiled bytecode, it may only exists temporarily while the bytecode is being produced. |
