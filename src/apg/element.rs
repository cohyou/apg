use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct Element(pub Vec<Vec<String>>);

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E");
        for (i, multiply) in self.0.iter().enumerate() {
            for (j, plus) in multiply.iter().enumerate() {
                write!(f, "{:?}", plus);
                if j < multiply.len() - 1 {
                    write!(f, ".");
                }
            }
            if i < self.0.len() - 1 {
                write!(f, "*");
            }
        }
        write!(f, "")
    }
}
