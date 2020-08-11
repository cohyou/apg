use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Element {
    E(String),
    Tuple(Box<Element>, Box<Element>),
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::E(e_name) => {
                write!(f, "E{:?}", e_name)            
            }
            Element::Tuple(e1, e2) => {
                write!(f, "({:?}, {:?})", e1, e2)
            }
        }
    }
}
