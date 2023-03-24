use std::collections::HashSet;
use std::fmt::Display;
use crate::{predicate::Predicate, variable::Variable, term::Term};

#[derive(Clone, Debug, PartialEq)]
pub struct Atom { // un predicat avec des arguments
    
    pub predicate: Predicate, // juste un nom qui est de type string pour nous
    pub args: Vec<Term> // soit des constantes, soit des variables, qu'on les stockes dans un vecteur
    
    /*
    le nombre d'arguments d'un predicat est fixe, on l'indique apres le nom du predicat
    homme/1
    
    */
    
}

/*
Les regles:
    Toutes les hypothese doivent etre satisfaiter pour pouvoir appliquer la regle
    et deduire la conclusion.
    Le "," a le sens d'une conjonction (le et logique).
    b :- a1, a2, ..., an. => si a1 et a2 et an, alors b

*/


impl Atom {
    pub fn variables(&self) -> HashSet<Variable> {
        // renvoie l'ensemble des variables d'un atome
        let mut res = HashSet::new();
        for arg in &self.args {
            for v in arg.variables() {
                res.insert(v);
            }
        };
        res
    } 
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.predicate)?;
        if self.args.len() != 0 {
            write!(f, "({}", self.args[0])?;
            for i in 1..self.args.len() {
                write!(f, ", {}", self.args[i])?;
            };
            write!(f, ")")?;
        };
        write!(f, "")
    }
}