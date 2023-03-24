use std::fmt::Display;

use crate::clause::Clause;

pub struct Program {
    pub clauses: Vec<Clause>
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for clause in &self.clauses {
            write!(f, "{}\n", clause)?;
        };
        write!(f, "")
    }
}