mod parser;
use crate::parser::lexer::Lexer;

fn main() {
    let input = "let";
    let mut lexer = Lexer::new(input);

    println!("{:#?}", lexer.next());

    println!("Hello, world!");
}
