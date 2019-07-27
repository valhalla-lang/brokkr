# Brokkr Bytecode Specification
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
