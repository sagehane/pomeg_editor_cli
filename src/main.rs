use std::{fs, io::Read};

use libpomeg::{DataStructure, Save, SaveStruct};

fn main() {
    let file = std::env::args().nth(1).expect("no file given"); // The program expects a file for an argument

    if fs::metadata(&file).unwrap().len() != 0x20000 {
        panic!("Invalid file size, should be 128 KiB");
    }

    let mut buffer = [0; 0x20000];

    std::fs::File::open(file)
        .unwrap()
        .read_exact(&mut buffer[..])
        .expect("could not read file");

    let gen3save = SaveStruct::from_save(Save::from_slice(&buffer));

    println!("{:#?}", gen3save);
}
