#![allow(dead_code)]

mod scanner;
mod util;
mod parser;
mod passes;
mod compiler;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::scanner::Scanner;
    use crate::parser::ast::ASTNodeVecExt;

    #[track_caller]
    fn parse(input: &str) -> String {
        let tokens = Scanner::new(input).scan();
        let ast = Parser::new(tokens.unwrap()).parse();
        ast.unwrap().display(0)
    }

    #[test]
    fn test_pemdas() {
        assert_eq!(
            parse("1 + 2 * 3 / 4 / 5 - 6 == 7 && 1 == 2 || 3 != 4"),
            "{\n    (|| (&& (== (- (+ 1 (/ (/ (* 2 3) 4) 5)) 6) 7) (== 1 2)) (!= 3 4))\n}\n"
        );
    }

    #[test]
    fn test_let_var_decl() {
        assert_eq!(
            parse("let x: int = 1;"),
            "{\n    (letvardecl x: int = 1)\n}\n"
        );
    }

    #[test]
    fn test_var_get() {
        assert_eq!(
            parse("1 + x"),
            "{\n    (+ 1 (var x))\n}\n"
        );
    }
}
