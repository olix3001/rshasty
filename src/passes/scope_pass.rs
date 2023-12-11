use crate::{util::scope::Scope, parser::ast::{ASTNode, BoxedASTNode}, scanner::{Token, TokenType}};

use super::Pass;

/// Compiler pass that resolves variable scopes and type information
pub struct ScopePass;

#[derive(Debug)]
pub enum ScopePassError {
    VariableNotDeclared {
        token: Token
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Used internally by ScopePass
struct VarInfo {
    ty: String
}

#[derive(Clone, Debug, PartialEq)]
/// Info about a type for metadata use
pub struct TypeInfo {
    ty: String
}

impl ScopePass {
    pub fn new() -> Self {
        Self {}
    }

    fn process_node(&mut self, node: &mut BoxedASTNode, scope: &mut Scope<VarInfo>) -> Result<(), ScopePassError> {
        match *node.node {
            ASTNode::VarDecl { ref name, ref ty, ref mut initializer } => {
                if let Some(ty) = ty {
                    scope.add(&name.lexeme, VarInfo { ty: ty.lexeme.clone() });
                    node.meta.insert(TypeInfo { ty: ty.lexeme.clone() });
                } else {
                    if let Some(ref mut initializer) = initializer {
                        self.process_node(initializer, scope)?;
                        node.meta.insert(TypeInfo { ty: initializer.meta.get::<TypeInfo>().unwrap().ty.clone() });
                    } else {
                        unimplemented!("Type inference from usage is not yet implemented, please specify a type for variable {}", name.lexeme)
                    }
                    scope.add(&name.lexeme, VarInfo { ty: node.meta.get::<TypeInfo>().unwrap().ty.clone() });
                }
            },
            ASTNode::Variable { ref name } => {
                if !scope.contains(&name.lexeme) {
                    return Err(ScopePassError::VariableNotDeclared { token: name.clone() });
                }
                node.meta.insert(TypeInfo { ty: scope.get(&name.lexeme).unwrap().ty.clone() });
            },
            ASTNode::Literal { ref value } => {
                node.meta.insert(TypeInfo { ty: match value.token_type {
                    TokenType::INTEGER => "int".to_string(),
                    TokenType::FLOATING => "float".to_string(),
                    TokenType::STRING => "string".to_string(),
                    TokenType::TRUE | TokenType::FALSE => "bool".to_string(),
                    _ => todo!("What should happen if we get to 'nil'?")
                } });
            },
            ASTNode::Grouping { ref mut expr } => {
                self.process_node(expr, scope)?;
                node.meta.insert(TypeInfo { ty: expr.meta.get::<TypeInfo>().unwrap().ty.clone() });
            },
            // TODO: Type inference for binary expressions (operator overloading should be implemented first)
            _ => {}
        }
        Ok(())
    }    
}

impl Pass for ScopePass {
    type Error = ScopePassError;
    fn process(&mut self, ast: &mut Vec<BoxedASTNode>) -> Result<(), Self::Error> {
        let mut scope = Scope::<VarInfo>::new();
        for node in ast {
            self.process_node(node, &mut scope)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::passes::Pass;
    use crate::scanner::Scanner;

    #[test]
    fn test_var_decl_type_explicit() {
        let tokens = Scanner::new("let x: int = 1;").scan();
        let ast = Parser::new(tokens.unwrap()).parse();
        let mut ast = ast.unwrap();
        super::ScopePass::new().process(&mut ast).unwrap();
        assert_eq!(
            ast[0].meta.get::<super::TypeInfo>().unwrap(),
            &super::TypeInfo { ty: "int".to_string() }
        );
    }

    #[test]
    fn test_var_get_type_explicit_def() {
        let tokens = Scanner::new("let x: int = 1; x").scan();
        let ast = Parser::new(tokens.unwrap()).parse();
        let mut ast = ast.unwrap();
        super::ScopePass::new().process(&mut ast).unwrap();
        assert_eq!(
            ast[1].meta.get::<super::TypeInfo>().unwrap(),
            &super::TypeInfo { ty: "int".to_string() }
        );
    }

    #[test]
    fn test_var_undefined() {
        let tokens = Scanner::new("x").scan();
        let ast = Parser::new(tokens.unwrap()).parse();
        let mut ast = ast.unwrap();
        assert!(super::ScopePass::new().process(&mut ast).is_err());
    }

    #[test]
    fn test_var_type_inference() {
        let tokens = Scanner::new("let x: int = 1; let y = x;").scan();
        let ast = Parser::new(tokens.unwrap()).parse();
        let mut ast = ast.unwrap();
        super::ScopePass::new().process(&mut ast).unwrap();
        assert_eq!(
            ast[1].meta.get::<super::TypeInfo>().unwrap(),
            &super::TypeInfo { ty: "int".to_string() }
        );
    }

    #[test]
    fn test_var_literal_inference() {
        let tokens = Scanner::new("let x = 1;").scan();
        let ast = Parser::new(tokens.unwrap()).parse();
        let mut ast = ast.unwrap();
        super::ScopePass::new().process(&mut ast).unwrap();
        assert_eq!(
            ast[0].meta.get::<super::TypeInfo>().unwrap(),
            &super::TypeInfo { ty: "int".to_string() }
        );
    }
}