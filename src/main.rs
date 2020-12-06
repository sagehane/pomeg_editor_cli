use pomeg_editor_cli::is_valid_save;

fn main() {
    let file = std::env::args().nth(1).expect("no file given"); // The program expects a file for an argument
    let content = std::fs::read(file).expect("could not read file"); // The program reads the file and makes it into a Vec<u8>

    if is_valid_save(&content) {
        println!("Savefile is valid");
    }
}
