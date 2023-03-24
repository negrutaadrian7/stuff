use std::{fmt::Display, collections::HashSet};
use crate::{atom::Atom, variable::Variable};

#[derive(Debug, PartialEq)]
pub enum RequestItem {
    Atom(Atom),
    Cut(usize)
}

impl Display for RequestItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestItem::Atom(a) => write!(f, "{}", a),
            RequestItem::Cut(n) => write!(f, "!{}", n)
        }
    }
}

impl RequestItem {
    fn variables(&self) -> HashSet<Variable> {
        match self {
            RequestItem::Atom(a) => a.variables(),
            RequestItem::Cut(_) => HashSet::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Request {
    // Une requête =  une liste d'atomes ou de coupures avec des numéros sur les coupures
    // On utilise Vec comme une pile, donc on liste en partant de la fin.
    // ex: la requête `:- a,b,c` sera représentée par `Request{goals:vec![c,b,a]}`
    pub goals: Vec<RequestItem>,
}

impl Request {

    pub fn variables(&self) -> HashSet<Variable> {
        let mut res = HashSet::new();
        for g in &self.goals {
            for v in g.variables() {
                res.insert(v);
            }
        };
        res
    }

}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut l = self.goals.iter().map(|x| format!("{}", x)).collect::<Vec<String>>();
        l.reverse();
        write!(f, ":- {}.", l.join(", "))
    }
}