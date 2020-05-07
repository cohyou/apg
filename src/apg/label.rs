use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Label(pub String);

impl fmt::Debug for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L{:?}", self.0)
    }
}
