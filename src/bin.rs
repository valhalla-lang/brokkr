use brokkr::vm::unmarshal;
use brokkr::vm::evaluation;

use std::env;
use std::io::prelude::*;
use std::{fs::File, path::Path};


pub fn main() -> Result<(), std::io::Error> {
    let mut args : Vec<String> = env::args().collect();
    args.remove(0);

    let files = args.iter().filter(|arg| Path::new(arg).exists());

    for file in files {
        #[cfg(feature="debug")]
        println!("Reading file {}...", file);

        let mut f = File::open(file)
            .expect("Could not open binary for reading.");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)
            .expect("Could not dump file contents to bytesteam.");

        let frame = unmarshal::parse_blob(&buffer);

        let mut env = evaluation::Environment::new();
        env.entry(frame);
        env.execute();
    }

    Ok(())
}
