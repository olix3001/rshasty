use nauvi::module::{Module, block::Statement, block::Block};

use crate::parser::ast::{BoxedASTNode, ASTNode};

/// Compiler that compiles a program to a javascript.
pub struct Compiler {
}

impl Compiler {
    pub fn new() -> Self {
        Self { }
    }

    pub fn compile_ast(&mut self, ast: &Vec<BoxedASTNode>, target: &mut impl std::io::Write) {
        let mut module = Module::create("comp_result");

        for node in ast {
            if let Some(statement) = Self::compile_node(node) {
                module.stmt(statement);
            }
        }

        module.generate_to(target);
    }

    pub fn compile_node(node: &BoxedASTNode) -> Option<Statement> {
        let n = node.node.borrow();
        match **n {
            ASTNode::Binary { ref left, ref operator, ref right } => {
                println!("binary");
                let left = Self::compile_node(&left)?;
                let right = Self::compile_node(&right)?;
                Some(Statement::Binary { 
                    left: left.boxed(),
                    operator: operator.lexeme.clone(),
                    right: right.boxed()
                })
            },
            ASTNode::Logical { ref left, ref operator, ref right } => {
                let left = Self::compile_node(&left)?;
                let right = Self::compile_node(&right)?;
                let mut block = Block::new(0);
                block.binary(left, &operator.lexeme, right);   
                Some(Statement::Block(Box::new(block)))
            },
            ASTNode::Unary { operator: _, right: _ } => {
                todo!("unary")
            },
            ASTNode::Literal { ref value } => {
                println!("literal");
                Some(Statement::Literal { value: value.lexeme.clone() })
            },
            ASTNode::Grouping { expr: _ } => {
                todo!("grouping")
            },
            // TODO: Add support for variable declarations and references (requires additional passes before compilation to resolve shadowing and such)
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::Compiler;

    #[track_caller]
    fn compile(code: &str) -> String {
        let scanner = crate::scanner::Scanner::new(code);
        let tokens = scanner.scan().unwrap();
        let parser = crate::parser::Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut compiler = Compiler::new();
        let mut result = Vec::new();
        compiler.compile_ast(&ast, &mut result);
        String::from_utf8(result).unwrap()
    }

    #[test]
    fn test_compile_binary_expr() {
        let result = compile("1 + 2 * 2");
        assert_eq!(
            "(1 + (2 * 2))\n",
            result
        )
    }
}