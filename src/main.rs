mod parser;
mod tokenizer;
use tokenizer::tokenize;

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let file_contents = std::fs::read_to_string(file_path).unwrap();

    let tokens = tokenize(file_contents);

    let asm = parser::parse(tokens);

    std::fs::write("out.asm", asm).unwrap();
}
