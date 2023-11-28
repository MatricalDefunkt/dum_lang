mod parser;
mod tokenizer;
mod utilities;

fn main() {
    // let file_path = std::env::args().nth(1).unwrap_or_else(|| {
    //     panic!("No file path provided!");
    // });
    let file_path = "./dum_lang_src/main.dl";
    let file_contents = std::fs::read_to_string(file_path).unwrap();

    let mut tokenizer = tokenizer::Tokenizer::new(file_contents);

    let tokens = tokenizer.tokenize();

    dbg!(tokens.clone());

    let asm = parser::parse(tokens);

    std::fs::write("out.asm", asm).unwrap();
}
