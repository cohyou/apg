use std::rc::Rc;
use std::fmt;
use super::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Unit,
    Inl(Rc<Value>, Rc<Type>),
    Inr(Rc<Type>, Rc<Value>),
    Pair(Rc<Value>, Rc<Value>),
    Prim(Rc<Value>),
    Id(Rc<Element>),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Unit => write!(f, "()"),
            Value::Inl(v, t) => write!(f, "{:?} + {:?}", v, t),
            Value::Inr(t, v) => write!(f, "{:?} + {:?}", t, v),
            Value::Pair(v1, v2) => write!(f, "{:?} * {:?}", v1, v2),
            Value::Prim(v) => write!(f, "P{:?}", v),
            Value::Id(e) => write!(f, "{:?}", e),
        }
    }
}
