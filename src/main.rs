use std::{convert::TryInto, fs, io::Read};

use clap::{App, Arg};
use libpomeg::{Save, SaveStruct};

fn main() {
    let matches = App::new("Pomeg Editor")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = matches.value_of("INPUT").unwrap();

    if fs::metadata(&file).unwrap().len() != 0x20000 {
        panic!("Invalid file size, should be 128 KiB");
    }

    let mut buffer = [0; 0x20000];

    std::fs::File::open(file)
        .unwrap()
        .read_exact(&mut buffer[..])
        .expect("could not read file");

    let gen3save = SaveStruct::from_save(Save::from_buffer(buffer.try_into().unwrap()));

    println!("{:#?}", gen3save);
}
