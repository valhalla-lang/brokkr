use brokkr::vm::unmarshal;

use std::env;
use std::io::prelude::*;
use std::{fs::File, path::Path};

pub fn main() -> Result<(), std::io::Error> {
    let mut args : Vec<String> = env::args().collect();
    args.remove(0);

    let files = args.iter().filter(|arg| Path::new(arg).exists());

    for file in files {
        #[cfg(debug_assertions)]
        println!("Reading file {}...", file);

        let mut f = File::open(file)
            .expect("Could not open binary for reading.");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)
            .expect("Could not dump file contents to bytesteam.");

        let _frame = unmarshal::parse_blob(&buffer);
    }

    Ok(())
}
