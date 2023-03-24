use std::{collections::{HashMap, HashSet}, fmt::Display, fmt::Debug};
use crate::{variable::Variable, term::Term, atom::Atom, clause::{Clause, ClauseRightItem}, request::{RequestItem, Request}};

#[derive(Clone, PartialEq, Debug)]
pub struct Substitution {
    bindings: HashMap<Variable, Term>
}

/*
Le resultat d'un programme Prolog est un ensemble de substitutions

*/



impl Substitution { // methodes de l'objet 
    
    pub fn identity() -> Self {
        // la substitution identité, notée [] dans le cours
        return Substitution { 
            bindings: HashMap::new() 
        };
    }
    
    
    pub fn substitution(x: Variable, t: Term) -> Self {
        // la substitution [X <- t]
        let mut res = Self::identity();
        res.bindings.insert(x, t);
        res
    }

    
    pub fn domain(&self) -> HashSet<&Variable> {
        // le domaine de la substitution (l'ensemble des variables substituées) 
        let mut res = HashSet::new();
        for (k,_) in self.bindings.iter() {
            res.insert(k);
        };
        res
    }


    pub fn apply_to_term(sigma: &Self, t: &Term) -> Term {
        // le résultat de l'application de la substitution au terme
        // si t=X et sigma = [X <- t', ...] alors t sigma = X[ X<- t', ...] = t'
        // si t=Y et Y n'est pas dans dom(sigma) alors t sigma = t = Y 
        // si t=c, alors t sigma = c sigma = c
        match t {
            Term::Variable(x) => {
                if let Some(v) = sigma.bindings.get(x) {
                    Substitution::apply_to_term(sigma, v)
                } else {
                    t.clone()
                }
            },
            Term::Constant(_) => t.clone(),
        }
    }



    pub fn apply_to_atom(sigma: &Self, atom: &Atom) -> Atom {
        // ce que l'on a noté Aσ dans le cours
        let new_predicate = atom.predicate.clone();
        let new_terms = atom.args.iter().map(|t| Self::apply_to_term(sigma, t)).collect();
        Atom {
            predicate: new_predicate,
            args: new_terms,
        }
    }

    pub fn apply_to_clause_item(subst: &Self, clause_item: &ClauseRightItem) -> ClauseRightItem {
        // applique la substitution a un élément de partie droite de clause
        match clause_item {
            ClauseRightItem::Atom(a) => ClauseRightItem::Atom(Substitution::apply_to_atom(subst, a)),
            ClauseRightItem::Cut => ClauseRightItem::Cut,
        }
    }


    pub fn apply_to_clause(subst: &Self, clause: &Clause) -> Clause {
        // applique la substitution à une clause
        let mut new_premises = Vec::new();
        for p in &clause.premises {
            new_premises.push(Substitution::apply_to_clause_item(subst, p));
        };
        
        Clause {
            goal: Substitution::apply_to_atom(subst, &clause.goal),
            premises: new_premises
        }
    }





    pub fn apply_to_request_item(subst: &Self, clause_item: &RequestItem) -> RequestItem {
        // applique la subtitution à un item droit de requête
        match clause_item {
            RequestItem::Atom(a) => RequestItem::Atom(Substitution::apply_to_atom(subst, a)),
            RequestItem::Cut(n) => RequestItem::Cut(*n),
        }
        
    }

    pub fn apply_to_request(subst: &Self, request: &Request) -> Request  {
        // applique la substitution à une requête
        let new_goals = request.goals.iter().map(|g| Substitution::apply_to_request_item(subst,g)).collect();
        Request { goals: new_goals }
    }


    pub fn compose(subst1: &Self, subst2: &Self) -> Self {
        let mut res = subst2.clone();
        for v in subst1.domain() {
            let t1 = Self::apply_to_term(subst1, &Term::Variable(v.clone()));
            let t2 = Self::apply_to_term(subst2, &t1);
            res.bindings.insert(v.clone(), t2);
        };
        res
    }
}





impl Display for Substitution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first_binding = true;
        for (v, t) in self.bindings.iter() {
            if !first_binding {
                write!(f, ", ")?;
            };
            first_binding = false;
            write!(f, "{}<-{}", v, t)?;
        };
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {

    use crate::{substitution::Substitution, variable::Variable, term::Term, constant::Constant};

    #[test]
    fn test_apply_to_term() {
        let c = Term::Constant(Constant::Identifier("c".to_string()));
        let x_ = Variable::of_str("x");
        let sigma = Substitution::substitution(x_, c.clone());
        let res = Substitution::apply_to_term(&sigma, &c);
        assert_eq!(res, c);
    }

    #[test]
    fn test_apply_to_atom() {
        // écrivez votre test ici
        todo!()
    }

    #[test]
    fn test_apply_to_clause() {
        // écrivez votre test ici
        todo!()
    }

    #[test]
    fn test_apply_to_request() {
        // écrivez votre test ici
        todo!()
    }


    #[test]
    fn test_compose1() {
        let x_ = Variable::of_str("X");
        let y_ = Variable::of_str("Y");
        let c1 = Term::Constant(Constant::Number(1.0));
        let c2 = Term::Constant(Constant::Number(2.0));
        let id = Substitution::identity();
        let sigma1 = Substitution::substitution(x_.clone(), c1.clone());
        let sigma2 = Substitution::substitution(y_.clone(), c2.clone());
        assert_eq!(Substitution::compose(&sigma1, &id), sigma1);
        assert_eq!(Substitution::compose(&id, &sigma1), sigma1);
        assert_eq!(Substitution::compose(&sigma1, &sigma2), Substitution::compose(&sigma2, &sigma1));
    }

    #[test]
    fn test_compose2() {
        let x_ = Variable::of_str("X");
        let y_ = Variable::of_str("Y");
        let x = Term::Variable(x_.clone());
        let y = Term::Variable(y_.clone());
        let z = Term::Variable(Variable::of_str("Z"));
        let c1 = Term::Constant(Constant::Number(1.0));
        let sigma1 = Substitution::substitution(x_.clone(), y.clone());
        let sigma2 = Substitution::substitution(y_.clone(), c1.clone());
        let sigma3 = Substitution::compose(&sigma1, &sigma2);
        print!("[test2] sigma3 = {}\n", sigma3);
        assert_eq!(Substitution::apply_to_term(&sigma3, &x), c1);
        assert_eq!(Substitution::apply_to_term(&sigma3, &y), c1);
        assert_eq!(Substitution::apply_to_term(&sigma3, &z), z);
        let sigma4 = Substitution::compose(&sigma2, &sigma1);
        print!("[test2] sigma4 = {}\n", sigma4);
        assert_eq!(Substitution::apply_to_term(&sigma4, &x), y);
        assert_eq!(Substitution::apply_to_term(&sigma4, &y), c1);
        assert_eq!(Substitution::apply_to_term(&sigma4, &z), z);
    }

    #[test]
    fn test_compose3() {
        let x_ = Variable::of_str("X");
        let c1 = Term::Constant(Constant::Number(1.0));
        let c2 = Term::Constant(Constant::Number(2.0));
        let sigma1 = Substitution::substitution(x_.clone(), c1.clone());
        let sigma2 = Substitution::substitution(x_.clone(), c2.clone());
        let sigma3 = Substitution::compose(&sigma1, &sigma2);
        assert_eq!(format!("{}", sigma3), "[X<-1]".to_string());
    }


}