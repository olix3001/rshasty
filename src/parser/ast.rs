use std::{fmt::Display, cell::RefCell, rc::Rc};

use crate::{scanner::Token, util::metacontainer::MetaContainer};

/// Boxed AST Node with metadata
#[derive(Debug, Clone)]
pub struct BoxedASTNode {
    pub node: Rc<RefCell<Box<ASTNode>>>,
    pub meta: MetaContainer,
}

impl BoxedASTNode {
    pub fn borrow(&self) -> std::cell::Ref<Box<ASTNode>> {
        self.node.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<Box<ASTNode>> {
        self.node.borrow_mut()
    }
}

impl From<ASTNode> for BoxedASTNode {
    fn from(node: ASTNode) -> Self {
        Self {
            node: Rc::new(RefCell::new(Box::new(node))),
            meta: MetaContainer::new(),
        }
    }
}

impl Display for BoxedASTNode {
    /// Show the AST in polish notation
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.node.borrow().fmt(f)
    }
}

/// Every possible AST Node is here.
#[derive(Debug)]
pub enum ASTNode {
    /// Expression with two operands (infix)
    Binary {
        left: BoxedASTNode,
        operator: Token,
        right: BoxedASTNode,
    },
    /// Logical expression with two operands (infix)
    Logical {
        left: BoxedASTNode,
        operator: Token,
        right: BoxedASTNode,
    },
    /// Expression with only one operand (prefix)
    Unary {
        operator: Token,
        right: BoxedASTNode,
    },
    /// Literal expression
    Literal {
        value: Token,
    },
    /// Grouping expression
    Grouping {
        expr: BoxedASTNode,
    },
    /// Variable declaration/definition
    VarDecl {
        name: Token,
        ty: Option<Token>,
        initializer: Option<BoxedASTNode>,
    },
    /// Variable
    Variable {
        name: Token,
    },
}

impl ASTNode {
    pub fn boxed(self) -> BoxedASTNode {
        BoxedASTNode::from(self)
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

            ASTNode::VarDecl { name, ty, initializer } => {
                write!(f, "(letvardecl {}{}{})",
                    name.lexeme,
                    if let Some(ty) = ty { format!(": {}", ty.lexeme) } else { String::new() },
                    if let Some(initializer) = initializer { format!(" = {}", initializer) } else { String::new() },
                )
            },
            ASTNode::Variable { name } => {
                write!(f, "(var {})", name.lexeme)
            }
        }
    }
}

pub trait ASTNodeVecExt {
    fn display(&self, indent: usize) -> String;
}

impl ASTNodeVecExt for Vec<ASTNode> {
    fn display(&self, indent: usize) -> String {
        let mut result = String::new();

        result.push_str(&format!("{}{{\n", "    ".repeat(indent)));
        for node in self {
            result.push_str(&format!("{}{}\n", "    ".repeat(indent+1), node));
        }
        result.push_str(&format!("{}}}\n", "    ".repeat(indent)));

        result
    }
}

impl ASTNodeVecExt for Vec<BoxedASTNode> {
    fn display(&self, indent: usize) -> String {
        let mut result = String::new();

        result.push_str(&format!("{}{{\n", "    ".repeat(indent)));
        for node in self {
            result.push_str(&format!("{}{}\n", "    ".repeat(indent+1), node));
        }
        result.push_str(&format!("{}}}\n", "    ".repeat(indent)));

        result
    }
}