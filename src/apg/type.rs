use std::rc::Rc;
use super::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Type {
    Zero,
    One,
    Sum(Rc<Type>, Rc<Type>),
    Product(Rc<Type>, Rc<Type>),
    Prim,
    Lbl(Rc<Label>),
}
