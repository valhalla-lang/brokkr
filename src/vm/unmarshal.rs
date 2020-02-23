/*!
 * # Parse/unmarshal bytecode compiled blobs.
 *
 * Files get directly parsed into stack frames, using an abundance
 * of pre-existing code in this project, forming them into well-understood
 * data structures, that are used throughout the program.
 *
 * The immediate next step from here is to pass the frames to the VM
 * and have it follow the instructions byte-for-byte.
 *
 */
use std::collections::HashSet;
use super::address::Address;
use super::opcodes;
use super::frame;
use super::frame::Instruction;

use num_traits::FromPrimitive;

const POINTER_BYTES : usize = std::mem::size_of::<usize>();

type Bytes = Vec<u8>;
type ByteSlice = [u8];

#[inline]
fn fix_slice_size<T, const N : usize>(slice : &[T]) -> &[T; N] {
    let ptr = slice.as_ptr() as *const [T; N];
    unsafe { &*ptr }
}

/// Functions that consume byte by byte, to reconstruct a code block.
mod eat {
    use super::*;

    /// Consume a null terminated string.
    pub fn null_string(mut i : usize, bytes : &ByteSlice) -> (usize, String) {
        let mut string : Bytes = vec![];
        while bytes[i] != 0x00 {
            string.push(bytes[i]);
            i += 1;
        }  // Trust these are valid bytes.
        let string = std::str::from_utf8(&string)
            .expect("Invalid utf8 bytes in null-terminated string. Bad bytecode.")
            .to_owned();

        return (i + 1, string);
    }

    fn consume_sized(mut i : usize, bytes : &ByteSlice) -> (usize, Bytes) {
        let size = bytes[i] as usize;
        i += 1;

        let mut padded = vec![0_u8; POINTER_BYTES];
        let slice = bytes[i..i + size].to_owned();
        for j in 0..size {
            padded[POINTER_BYTES - j - 1] = slice[size - j - 1];
        }

        (i + size, padded)
    }

    #[derive(Debug)]
    pub struct Egg {
        a : f64,
        b : String
    }

    fn constant(mut i : usize, bytes : &ByteSlice) -> (usize, Address) {
        let const_type = bytes[i];
        i += 1;
        return match const_type {
            // Parse number-types
            0x01..=0x03 => {
                let (i, bytes_slice) = consume_sized(i, bytes);
                let bytes_slice = fix_slice_size::<u8, POINTER_BYTES>(&bytes_slice[..POINTER_BYTES]);
                let value = Address(usize::from_be_bytes(*bytes_slice));
                (i, value)
            },
            // Parse Strings
            0x04 => {
                let (i, bytes_slice) = consume_sized(i, bytes);
                let bytes_slice = fix_slice_size::<u8, POINTER_BYTES>(&bytes_slice[..POINTER_BYTES]);
                let str_len = usize::from_be_bytes(*bytes_slice);

                // Store string on heap, `Address` holds a raw pointer to it.
                let string = Address::new(std::str::from_utf8(&bytes[i..i + str_len])
                    .expect("Invalid utf8 bytes in string. Bad bytecode."));
                (i + str_len, string)
            }
            _ => panic!(format!(
                "Type-specifier-prefix ({:x}) is not recognised.",
                const_type))
        }
    }

    pub fn constants(mut i : usize, bytes : &ByteSlice) -> (usize, Vec<Address>) {
        // Constant blocks are expected to start with `0x11`.
        #[cfg(debug_assertions)]
        assert_eq!(bytes[i], 0x11);
        i += 1;

        let mut consts : Vec<Address> = vec![];
        while bytes[i] != 0x00 {
            let (j, void) = constant(i, bytes);
            i = j;
            consts.push(void);
        }
        return (i + 1, consts);

    }

    /// Parse local variable names (null terminated strings).
    pub fn locals(mut i : usize, bytes : &ByteSlice) -> (usize, HashSet<String>) {
        let  mut set : HashSet<String> = HashSet::new();
        #[cfg(debug_assertions)]
        assert_eq!(bytes[i], 0x12);
        i += 1;

        while bytes[i] != 0x00 {  // Read strings until end of block.
            let (j, local) = eat::null_string(i, bytes);
            set.insert(local);
            i = j;
        }

        (i + 1, set)
    }

    pub fn instructions(mut i : usize, bytes : &ByteSlice) -> (usize, Vec<Instruction>) {
        let mut instrs : Vec<Instruction> = vec![];
        #[cfg(debug_assertions)]
        assert_eq!(bytes[i], 0x13);
        i += 1;

        while bytes[i] != 0x00 {
            instrs.push(Instruction::from(bytes[i]));
            let maybe_instr : Option<opcodes::Operators> =
                FromPrimitive::from_usize(bytes[i] as usize);
            if let Some(instr) = maybe_instr {
                // If the opcode takes an operand (u16), consume this too.
                if instr.takes_operand() {
                    i += 2;
                    let operand = (u16::from(bytes[i - 1]) << 8)
                        + u16::from(bytes[i]);
                    instrs.push(Instruction::from(operand));
                }
            }
            i += 1;
        }

        instrs.push(Instruction(0));
        (i, instrs)
    }

    /// Parse whole code-block.
    pub fn block(i : usize, bytes : &ByteSlice) -> (usize, frame::Frame) {
        // Parse source filename.
        let (i, filename) = eat::null_string(i, bytes);
        // Parse module name.
        let (i, module) = eat::null_string(i, bytes);
        // Parse max evaluation-stack depth.
        let stack_depth = (u16::from(bytes[i]) << 8) + u16::from(bytes[i + 1]);
        let i = i + 2;
        // Parse constants.
        let (i, constants) = eat::constants(i, bytes);
        // Parse locals.
        let (i, locals) = eat::locals(i, bytes);
        // Parse instructions.
        let (i, instructions) = eat::instructions(i, bytes);

        // Construct call-frame.
        let stack_frame = frame::Frame::new(
            filename, module, constants,
            locals, instructions, stack_depth);

        return (i, stack_frame);
    }
}

#[must_use]
pub fn parse_blob(bytes : &ByteSlice) -> frame::Frame {
    let mut i : usize = 0;
    // Parse compiler version number.
    let _version = bytes[0..2].as_ref();
    i += 3;

    // Parse primary/root code block.
    let (_, stack_frame) = eat::block(i, bytes);

    #[cfg(feature="debug")]
    println!("{:#?}", stack_frame);

    // If `stack_frame.constants[2]` is a pointer to a string, then, to use
    // it in Rust, all you have to do is:
    // ```
    //   let string : &str = unsafe {
    //       *(stack_frame.constants[2].0 as *const &str)
    //   };
    //   println!("str: {}", string);
    // ```
    // Or even better:
    // ```
    //   let string : &str = unsafe { stack_frame.constants[2].deref() };
    //   println!("str: {}", string);
    // ```

    return stack_frame;
}

/* === ROOT BLOB FORMAT ===:
 *  | VERSION [u8; 3]
 *  | === MARSHALLED CODE BLOCK FORMAT ===:
 *  |  | source-filename [u8; x] (abs path, null terminated, utf8)
 *  |  | module-name [u8; x] (null terminated, utf8)
 *  |  | stack-depth [u8; 2]
 *  |  |
 *  |  | CONSTANTS [u8; x] (block begin: 0x11 byte)
 *  |  |     (can contain other marshalled code blocks)
 *  |  | (block end: 0x00)
 *  |  | LOCAL NAMES [u8; x] (block begin: 0x12)
 *  |  |     (contains null terminated strings)
 *  |  | (block end: 0x00)
 *  |  | INSTRUCTION CODES [u8; x] (block begin: 0x13)
 *  |  |     (contains stream of operators and operands)
 *  |  | (block end: 0x00 (EOI))
 */
