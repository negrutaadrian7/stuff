use std::collections::HashSet;
use std::fmt::Display;
use crate::constant::Constant;
use crate::variable::Variable;

#[derive(Clone, PartialEq, Debug)]
pub enum Term {
    
    Constant(Constant),
    Variable(Variable) // commencent par une lettre majuscule

}

impl Term {
    pub fn variables(&self) -> HashSet<Variable> {
        // renvoie l'ensemble des variables d'un terme
        match self {
            Term::Constant(_) => HashSet::new(),
            Term::Variable(v) => {
                let mut res = HashSet::new();
                res.insert(v.clone());
                res
            },
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Constant(c) => write!(f, "{}", c),
            Term::Variable(v) => write!(f, "{}", v),
        }
    }
}