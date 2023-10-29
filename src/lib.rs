#![allow(dead_code)]

mod scanner;
mod util;
mod parser;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    #[test]
    fn it_works() {
        let tokens = Scanner::new("2 + 2 * 2 == 6 && 3 == 3 || 7 == 8 && 2 - 2 == 0").scan();
        println!("{:#?}", tokens);
        let ast = Parser::new(tokens.unwrap()).parse();
        panic!("{}", ast.unwrap());
    }
}
