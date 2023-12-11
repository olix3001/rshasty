use crate::{util::scope::Scope, parser::ast::ASTNode, scanner::Token};

use super::{Pass, ASTNodeMeta};

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

    fn process_node(&mut self, node: &mut ASTNodeMeta, scope: &mut Scope<VarInfo>) -> Result<(), ScopePassError> {
        match node.node {
            ASTNode::VarDecl { ref name, ref ty, ref initializer } => {
                if let Some(ty) = ty {
                    scope.add(&name.lexeme, VarInfo { ty: ty.lexeme.clone() });
                    node.meta.insert(TypeInfo { ty: ty.lexeme.clone() });
                } else {
                    unimplemented!("Type inference not implemented yet, please specify a type for variable {}", name.lexeme);
                }
            },
            ASTNode::Variable { ref name } => {
                if !scope.contains(&name.lexeme) {
                    return Err(ScopePassError::VariableNotDeclared { token: name.clone() });
                }
                node.meta.insert(TypeInfo { ty: scope.get(&name.lexeme).unwrap().ty.clone() });
            },
            _ => {}
        }
        Ok(())
    }    
}

impl Pass for ScopePass {
    type Error = ScopePassError;
    fn process(&mut self, ast: &mut Vec<ASTNodeMeta>) -> Result<(), Self::Error> {
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
    use crate::passes::{Pass, ASTNodeMeta};
    use crate::scanner::Scanner;

    #[test]
    fn test_var_decl_type_explicit() {
        let tokens = Scanner::new("let x: int = 1;").scan();
        let ast = Parser::new(tokens.unwrap()).parse();
        let ast = ast.unwrap();
        let mut ast = ASTNodeMeta::from_vec(ast);
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
        let ast = ast.unwrap();
        let mut ast = ASTNodeMeta::from_vec(ast);
        super::ScopePass::new().process(&mut ast).unwrap();
        assert_eq!(
            ast[1].meta.get::<super::TypeInfo>().unwrap(),
            &super::TypeInfo { ty: "int".to_string() }
        );
    }
}