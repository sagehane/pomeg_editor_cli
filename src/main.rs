use pomeg_editor_cli::check_save;

fn main() {
    let file = std::env::args().nth(1).expect("no file given"); // The program expects a file for an argument
    let content = std::fs::read(file).expect("could not read file"); // The program reads the file and makes it into a Vec<u8>

    match check_save(&content) {
        true => println!("Savefile is valid!"),
        false => println!("Savefile is invalid!"),
    }
}
