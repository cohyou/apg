use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct Label(pub Vec<Vec<String>>);

impl fmt::Debug for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L");
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
