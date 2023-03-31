use std::fmt::Display;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct Variable {
    pub identifier: String,
    pub variant: usize
}

impl Variable {

    pub fn of_str(s: &str) -> Self {
        Variable {
            identifier: s.to_string(),
            variant: 0
        }
    }


    pub fn with_variant(&self, variant: usize) -> Variable {
        Variable {
            identifier: self.identifier.clone(),
            variant,
        }
    }

}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.variant == 0 { write!(f, "{}", self.identifier) }
        else { write!(f, "{}_{}", self.identifier, self.variant)}
    }
}