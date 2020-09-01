use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct Element(pub Vec<Vec<String>>);

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E{:?}", self.0)
    }
}
