use pomeg_editor_cli::is_valid_save;
use std::io::Read;

fn main() {
    let file = std::env::args().nth(1).expect("no file given"); // The program expects a file for an argument
    let mut buffer = [0; 0x20000];

    std::fs::File::open(file)
        .unwrap()
        .read_exact(&mut buffer[..])
        .expect("could not read file");

    if is_valid_save(&buffer) {
        println!("Savefile is valid");
    }
}
