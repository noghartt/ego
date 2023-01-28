use super::location::Spanned;

#[derive(Debug)]
pub enum Operation {
    Add,
    Mult,
    Slash,
    Minus,
}

pub type Expr = Spanned<ExprKind>;

#[derive(Debug)]
pub struct LetNode {
    pub name: String,
    pub value: Box<Expr>,
    pub next: Box<Expr>,
}

#[derive(Debug)]
pub struct BinaryNode {
    pub operation: Operation,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct UnaryNode {
    pub operation: Operation,
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub enum ExprKind {
    Let(LetNode),
    Ident(String),
    Binary(BinaryNode),
    Unary(UnaryNode),
    // TODO: Turn it into an i64
    Int(usize),
}
