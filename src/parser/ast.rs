use std::fmt::Display;

use crate::scanner::Token;

/// Every possible AST Node is here.
#[derive(Debug)]
pub enum ASTNode {
    /// Expression with two operands (infix)
    Binary {
        left: Box<ASTNode>,
        operator: Token,
        right: Box<ASTNode>,
    },
    /// Logical expression with two operands (infix)
    Logical {
        left: Box<ASTNode>,
        operator: Token,
        right: Box<ASTNode>,
    },
    /// Expression with only one operand (prefix)
    Unary {
        operator: Token,
        right: Box<ASTNode>,
    },
    /// Literal expression
    Literal {
        value: Token,
    },
    /// Grouping expression
    Grouping {
        expr: Box<ASTNode>,
    }
}

impl ASTNode {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Display for ASTNode {
    /// Show the AST in polish notation
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::Binary { left, operator, right } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            ASTNode::Logical { left, operator, right } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            ASTNode::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
            ASTNode::Literal { value } => {
                write!(f, "{}", value.lexeme)
            }
            ASTNode::Grouping { expr } => {
                write!(f, "({})", expr)
            }
        }
    }
}