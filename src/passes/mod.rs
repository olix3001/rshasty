use crate::{parser::ast::ASTNode, util::metacontainer::MetaContainer};

mod scope_pass;

/// Type that contains ASTNode and its corresponding metadata
pub struct ASTNodeMeta {
    pub node: ASTNode,
    pub meta: MetaContainer,
}

/// Layer for processing ASTNodeMeta
pub trait Pass {
    type Error;
    fn process(&mut self, ast: &mut Vec<ASTNodeMeta>) -> Result<(), Self::Error>;
}

impl ASTNodeMeta {
    pub fn new(node: ASTNode, meta: MetaContainer) -> Self {
        Self { node, meta }
    }

    pub fn from_vec(nodes: Vec<ASTNode>) -> Vec<Self> {
        nodes.into_iter().map(|x| Self::from(x)).collect()
    }
}

impl From<ASTNode> for ASTNodeMeta {
    fn from(node: ASTNode) -> Self {
        Self {
            node,
            meta: MetaContainer::new(),
        }
    }
}

impl From<ASTNodeMeta> for ASTNode {
    fn from(node: ASTNodeMeta) -> Self {
        node.node
    }
}

impl From<Box<ASTNode>> for ASTNodeMeta {
    fn from(node: Box<ASTNode>) -> Self {
        Self {
            node: *node,
            meta: MetaContainer::new(),
        }
    }
}

impl From<ASTNodeMeta> for Box<ASTNode> {
    fn from(node: ASTNodeMeta) -> Self {
        Box::new(node.node)
    }
}