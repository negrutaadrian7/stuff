use std::{fmt::Display, collections::HashSet};
use crate::{atom::Atom, variable::Variable};

// QU'EST-CE QU'UNE CLAUSE?
// une clause positive est un fait ou une règle. On identifie un fait à une règle sans prémisses.
//       mortel(socrate).
//       mortel(socrate) :- .
// Dans la partie à droite de `:-` de la clause on trouve des "items"
// ce sont ou bien des atomes, ou bien la coupure `!`


#[derive(Clone, Debug, PartialEq)]

pub enum ClauseRightItem {
    Atom(Atom),
    Cut
}

impl ClauseRightItem {
    pub fn variables(&self) -> HashSet<Variable> {
        // l'ensemble des variables d'un item de clause
        match self {
            ClauseRightItem::Atom(a) => a.variables(),
            ClauseRightItem::Cut => HashSet::new(),
        }
    }
}

impl Display for ClauseRightItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClauseRightItem::Atom(a) => write!(f, "{}", a),
            ClauseRightItem::Cut => write!(f, "!")
        }
    }
}

#[derive(Clone)]

pub struct Clause {
    // Une clause positive: peut être un fait (si pas de prémisses) ou une règle
    pub goal: Atom,
    pub premises: Vec<ClauseRightItem>
}

impl Clause {
    pub fn variables(&self) -> HashSet<Variable> {
        let mut res = self.goal.variables();
        for p in &self.premises {
            for v in p.variables() {
                res.insert(v);
            }
        };
        res
    }
}

impl Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.goal)?;
        if self.premises.len() != 0 {
            write!(f, " :- {}", self.premises[0])?;
            for i in 1..self.premises.len() {
                write!(f, ", {}", self.premises[i])?;
            };
        };
        write!(f, ".")
    }
}