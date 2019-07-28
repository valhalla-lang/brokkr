# Brokkr Bytecode Specification

Sizes are as follows:

| Type     | Size                     |
|----------|--------------------------|
| Operator | 1 Byte  (8  Bits, `u8`)  |
| Operand  | 2 Bytes (16 Bits, `u16`) |

## Bytecodes
| Byte | Name | Operands | Description |
|:---:|:---:|---|---|
| `00000000` | `HALT` | 1 — Exit Code (Integer) | Stops execution. |
| `00000001` | `PUSH_CONST` | 1 — Unsigned index to constants table | Pushes a constant onto the stack. Constant is specified by a given index to the constant-table, containing all local constant. |
| `00000010` | `PUSH_LOCAL` | 1 — Local variable index (Unsigned) | Pushes a local variable (e.g. a function param) onto the stack. The local variable is identified by an index operand, to where it is in the current symbol table. |
| `00000011` | `PUSH_SUPER` | 1 — Name of super-scoped variable (String) | When a variable is out of the local scope, load it onto the stack by searching up the super-scope by its name, (much slower than `PUSH_LOCAL`). |
| `00000100` | `POP` |  | Pops the top element off of the stack. |
| `00000101` | `STORE_LOCAL` | 1 — Index to store local variable in table (Unsigned) | Pops the top value and stores it in the local symbol table at a given index. |
| `00000110` | `STORE_SUPER` | 1 — Name of super-scoped variable (String) | When a variable is out of the local scope, pop it off of the stack to store it in the super-scope searching up the frames for its name, (much slower than `STORE_LOCAL`). |
| `00000110` | `DUP` |  | Duplicates what was on the top of the stack, by popping it off and then pushing two copies. |
| `00000111` | `DUP_N` | 1 — The number of duplicates | Duplicates the top value on the stack N times. |
| `00001000` | `SWAP` |  | Swaps the position of the top two stack elements. |
| `00101000` | `N_ADD` |  | Addition between two pointer-sized unsigned integers. Pops top two elements from the stacks and adds them together, pushing the result. |
| `00101001` | `I_ADD` |  | Addition between two pointer-sized signed integers. Pops top two elements from the stacks and adds them together, pushing the result. |
| `00101010` | `R_ADD` |  | Addition between two 64-bit floating-point numbers. Pops top two elements from the stacks and adds them together, pushing the result. |
| `00101011` | `U_ADD` |  | Addition between two values of unknown types, found out at runtime. Pops top two elements from the stacks and adds them together, pushing the result. |
| `00101100` | `CONCAT` |  | Works like add, but concatenates strings. |
| `00101101` | `N_SUB` |  | Subtraction between two pointer-sized unsigned integers. Pops top two elements from the stacks and subtracts them together, pushing the result. |
| `00101110` | `I_SUB` |  | Subtraction between two pointer-sized signed integers. Pops top two elements from the stacks and subtracts them together, pushing the result. |
| `00101111` | `R_SUB` |  | Subtraction between two 64-bit floating-point numbers. Pops top two elements from the stacks and subtracts them together, pushing the result. |
| `00110000` | `U_SUB` |  | Subtraction between two values of unknown types, found out at runtime. Pops top two elements from the stacks and subtracts one from the other, pushing the result. |
| `00110001` | `N_MUL` |  | Multiplication between two pointer-sized unsigned integers. Pops top two elements from the stacks and multiplies them together, pushing the result. |
| `00110010` | `I_MUL` |  | Multiplication between two pointer-sized signed integers. Pops top two elements from the stacks and multiplies them together, pushing the result. |
| `00110011` | `R_MUL` |  | Multiplication between two 64-bit floating-point numbers. Pops top two elements from the stacks and multiplies them together, pushing the result. |
| `00110100` | `U_MUL` |  | Multiplication between two values of unknown types, found out at runtime. Pops top two elements from the stacks and multiplies them together, pushing the result. |
| `00110101` | `N_DIV` |  | Division between two pointer-sized unsigned integers. Pops top two elements from the stacks and finds their quotient, pushing the result. |
| `00110110` | `I_DIV` |  | Division between two pointer-sized signed integers. Pops top two elements from the stacks and finds their quotient, pushing the result. |
| `00110111` | `R_DIV` |  | Division between two 64-bit floating-point numbers. Pops top two elements from the stacks and finds their quotient, pushing the result. |
| `00111000` | `U_DIV` |  | Division between two values of unknown types, found out at runtime. Pops top two elements from the stacks and finds their quotient, pushing the result. |
