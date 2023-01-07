use std::ops::Range;

pub type Point = usize;
pub type Span = Range<usize>;

#[derive(Default, Debug, Clone)]
pub struct Spanned<T> {
    pub data: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(start: Point, end: Point, data: T) -> Spanned<T> {
        Spanned {
            data,
            span: start..end,
        }
    }
}
