use std::{collections::{HashMap, HashSet}, fmt::Display, fmt::Debug};
use crate::{variable::Variable, term::Term, atom::Atom, clause::{Clause, ClauseRightItem}, request::{RequestItem, Request}};

#[derive(Clone, PartialEq, Debug)]
pub struct Substitution {
    bindings: HashMap<Variable, Term>
}

/*

Le resultat d'un programme Datalog est un ensemble de substitutions
a une variable - la substitution associe un terme

On remplace chaque variable du domaine de la substitution par le terme associe.
Les autres variables restent inchangees.

*/



impl Substitution  { // methodes de l'objet 
    
    pub fn identity() -> Self {
        // la substitution identité, notée [] dans le cours
        return Substitution { 
            bindings: HashMap::new() 
        };
    }
    
    
    pub fn substitution(x: Variable, t: Term) -> Self {
        // la substitution [X <- t]
        // representee par une HashMap qui a une variable associe un terme.
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

        // term c'est soit une constante, soit une variable
        // donc pattern-matching

        match t {
            Term::Variable(x) => { // if we have a variable that is replaced by another one we'll call this function in a recursive way
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
        // applies the given substitution sigma to an atom and return the resulting atom.
        // ce que l'on a noté Aσ dans le cours
        
        // On peut appliquer une substitution à un atome: on remplace chaque variable du 
        // domaine de la substitution par le terme associé. 
        // Les autres variables restent inchangées.


        let new_predicate = atom.predicate.clone(); // le predicate reste le meme
        let new_terms = atom.args.iter().map(|t| Self::apply_to_term(sigma, t)).collect(); // pour chaque term t on fait un appel au fonction apply_to_term
        
        Atom { // return the result atom
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
        let mut new_premises = Vec::new(); // creation d'un nouveau vecteur qui va contenir les nouveaux premises
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
        // pour chaque element du vecteur goals on applique la fonction apply_to_request_item
        // on sait qu'il s'agit des elements RequestItem dans le vecteur
        let new_goals = request.goals.iter().map(|g| Substitution::apply_to_request_item(subst,g)).collect(); 
        Request { goals: new_goals }
    }

    // Applying the first map to each variable in the second map
    // return a new map that will combine the results
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

    use crate::{substitution::Substitution, variable::Variable, term::Term, constant::Constant, atom::Atom, clause::{Clause, ClauseRightItem}, request::{Request, RequestItem}};

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
        let c = Term::Constant(Constant::Identifier("c".to_string()));
        let x_1 = Variable::of_str("x");
        let y_1 = Variable::of_str("y");
        let z_1 = Variable::of_str("z");
        let sigma = Substitution::substitution(x_1.clone(), c.clone());
        let a = Atom::new("p", vec![Term::Variable(x_1.clone()), Term::Variable(y_1.clone()), Term::Variable(z_1.clone())]);
        let res = Substitution::apply_to_atom(&sigma, &a);
        assert_eq!(res, Atom::new("p", vec![c.clone(), Term::Variable(y_1.clone()), Term::Variable(z_1.clone())]));
    
        // Test with variables in the atom
        let x_2 = Variable::of_str("x");
        let w_2 = Variable::of_str("w");
        let a2 = Atom::new("q", vec![Term::Variable(x_2.clone()), Term::Variable(w_2.clone())]);
        let sigma2 = Substitution::substitution(x_2.clone(), c.clone());
        let res2 = Substitution::apply_to_atom(&sigma2, &a2);
        assert_eq!(res2, Atom::new("q", vec![c.clone(), Term::Variable(w_2.clone())]));
    }



    #[test]
    fn test_apply_to_clause() {
        // écrivez votre test ici
        let x_ = Variable::of_str("x");
        let y_ = Variable::of_str("y");
        let z_ = Variable::of_str("z");
        let c = Term::Constant(Constant::Identifier("c".to_string()));
        let d = Term::Constant(Constant::Identifier("d".to_string()));
        let e = Term::Constant(Constant::Identifier("e".to_string()));
        let a1 = Atom::new("p", vec![Term::Variable(x_.clone()), Term::Variable(y_.clone()), c.clone()]);
        let a2 = Atom::new("q", vec![Term::Variable(y_.clone()), Term::Variable(z_)]);
        let a3 = Atom::new("r", vec![c.clone(), d.clone()]);
        let clause = Clause { goal: a1.clone(), premises: vec![ClauseRightItem::Atom(a2.clone()), ClauseRightItem::Atom(a3.clone())] };
        let sigma = Substitution::substitution(y_.clone(), d.clone());
        let res = Substitution::apply_to_clause(&sigma, &clause);
        let expected_goal = Atom::new("p", vec![Term::Variable(x_.clone()), d.clone(), c.clone()]);
        let expected_premises = vec![ClauseRightItem::Atom(Substitution::apply_to_atom(&sigma, &a2)), ClauseRightItem::Atom(Substitution::apply_to_atom(&sigma, &a3))];
        let expected = Clause { goal: expected_goal, premises: expected_premises };
        assert_eq!(res.premises, expected.premises);
    }



    #[test]
    fn test_apply_to_request() {
        let x_ = Variable::of_str("x");
        let c = Term::Constant(Constant::Identifier("c".to_string()));
        let d = Term::Constant(Constant::Identifier("d".to_string()));
        let a1 = Atom::new("p", vec![Term::Variable(x_), c.clone()]);
        let a2 = Atom::new("q", vec![c.clone(), Term::Variable(Variable::of_str("z"))]); 
        let request = Request{ goals: vec![RequestItem::Atom(a1.clone()), RequestItem::Atom(a2.clone())] };
        let sigma = Substitution::substitution(Variable::of_str("z"), d.clone()); 
        let res = Substitution::apply_to_request(&sigma, &request);
        let expected_goals = vec![RequestItem::Atom(Substitution::apply_to_atom(&sigma, &a1)), RequestItem::Atom(Substitution::apply_to_atom(&sigma, &a2))];
        let expected = Request{ goals: expected_goals };
        assert_eq!(res, expected);
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