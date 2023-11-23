mod parser;
mod tokenizer;
mod utilities;
use tokenizer::tokenize;

fn main() {
    // let file_path = std::env::args().nth(1).unwrap_or_else(|| {
    //     panic!("No file path provided!");
    // });
    let file_path = "./dum_lang_src/main.dl";
    let file_contents = std::fs::read_to_string(file_path).unwrap();

    let tokens = tokenize(file_contents);

    let asm = parser::parse(tokens);

    std::fs::write("out.asm", asm).unwrap();
}
