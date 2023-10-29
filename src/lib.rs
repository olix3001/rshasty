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
    use crate::util::HastyError;
    use super::*;

    #[test]
    fn it_works() {
        let tokens = Scanner::new("(2 + 2 * 2 / 5 - 2").scan();
        println!("{:#?}", tokens);
        let ast = Parser::new(tokens.unwrap()).parse();
        panic!("{}", ast.unwrap_err().as_hasty_error_string());
    }
}
