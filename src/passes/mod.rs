use crate::parser::ast::BoxedASTNode;

/// Layer for processing ASTNodeMeta
pub trait Pass {
    type Error;
    type AdditionalData;
    fn process(&mut self, ast: &mut Vec<BoxedASTNode>) -> Result<Self::AdditionalData, Self::Error>;
}