use super::location::Spanned;

#[derive(Debug)]
pub enum Operation {
    Add,
    Mult,
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
pub enum ExprKind {
    Let(LetNode),
    Ident(String),
    Binary(BinaryNode),
    // TODO: Turn it into an i64
    Int(usize),
}
