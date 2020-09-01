use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct Label(pub Vec<Vec<String>>);

impl fmt::Debug for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L{:?}", self.0)
    }
}
