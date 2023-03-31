use std::ops::Sub;

use crate::{term::Term, variable::Variable, atom::Atom, substitution::Substitution};

pub fn unify(term1: &Term, term2: &Term) -> Option<Substitution> {
    match (term1, term2) {
        // If the two terms are constants or the same variable, they unify trivially
        (Term::Constant(c1), Term::Constant(c2)) if c1 == c2 => Some(Substitution::identity()),
        (Term::Variable(v1), Term::Variable(v2)) if v1 == v2 => Some(Substitution::identity()),

        // If one of the terms is a variable, unify it with the other term
        (Term::Variable(v), t) => Some(Substitution::substitution(v.clone(), t.clone())),

        (t, Term::Variable(v)) => Some(Substitution::substitution(v.clone(), t.clone())),

        // If the two terms have different constants, they cannot unify
        (Term::Constant(_), _) | (_, Term::Constant(_)) => None,

        // If the two terms are variables with different identifiers, they cannot unify
        (Term::Variable(v1), Term::Variable(v2)) if v1.identifier != v2.identifier => None,

        // If the two terms are variables with the same identifier, they unify trivially
        (Term::Variable(_), _) | (_, Term::Variable(_)) => Some(Substitution::identity()),
    }
}




pub fn mgu(atom1: &Atom, atom2: &Atom) -> Option<Substitution> {
    // renvoie l'unificateur le plus général des deux atomes, s'il existe, sinon None
    if atom1.predicate != atom2.predicate {
        return None;
    }
    let mut sigma = Substitution::identity();

    for (arg1, arg2) in atom1.args.iter().zip(atom2.args.iter()){ // will return pairs made of (arg1, arg2) from the 2 vectors
        let sigma_prime = unify(&Substitution::apply_to_term(&sigma, arg1), &Substitution::apply_to_term(&sigma, arg2))?;
        sigma = Substitution::compose(&sigma, &sigma_prime);
    }
    
    Some(sigma)
}



#[cfg(test)]
mod tests {
    use crate::{substitution::Substitution, variable::Variable, term::Term, constant::Constant, atom::Atom};
    use crate::unification::mgu;

    #[test]
    fn test1() {
        let c1 = Term::Constant(Constant::Number(1.0));
        let c2 = Term::Constant(Constant::Number(2.0));
        let x = Term::Variable(Variable::of_str("X"));
        let y = Term::Variable(Variable::of_str("Y"));
        let a = |x: &Term| Atom{predicate:"a".to_string(), args:vec![x.clone()]};
        let b = |x: &Term| Atom{predicate:"b".to_string(), args:vec![x.clone()]};
        let a2 = |x: &Term, y: &Term| Atom{predicate:"a".to_string(), args:vec![x.clone(), y.clone()]};
        let atom1 = a(&c1);
        let atom2 = a(&x);
        let atom3 = b(&c1);
        let atom4 = a(&c2);
        let atom5 = a2(&c1, &c1);
        assert_eq!(mgu(&atom1, &atom1), Some(Substitution::identity()));
        assert_eq!(mgu(&atom1, &atom2), Some(Substitution::substitution(Variable::of_str("X"), c1)));
        assert_eq!(mgu(&atom1, &atom3), None);
        assert_eq!(mgu(&atom1, &atom4), None);
        assert_eq!(mgu(&atom1, &atom5), None);
        assert_eq!(mgu(&atom5, &atom1), None);
    }

    #[test]
    fn test2(){
        let c1 = Term::Constant(Constant::Number(1.0));
        let c2 = Term::Constant(Constant::Number(2.0));
        let x_ = Variable::of_str("X");
        let y_ = Variable::of_str("Y");
        let z_ = Variable::of_str("Z");
        let x = Term::Variable(x_.clone());
        let y = Term::Variable(y_.clone());
        let z = Term::Variable(z_.clone());
        let a = |x: &Term, y: &Term, z: &Term| Atom{predicate:"a".to_string(), args:vec![x.clone(), y.clone(), z.clone()]};
        let atom1 = a(&c1, &c2, &c1);
        let atom2 = a(&x, &y, &z);
        let atom3 = a(&x, &y, &x);
        let atom4 = a(&x, &y, &y);
        let sigma1 = Substitution::substitution(x_.clone(), c1.clone());
        let sigma2 = Substitution::substitution(y_.clone(), c2.clone());
        let sigma3 = Substitution::substitution(z_.clone(), c1.clone());
        let sigma4 = Substitution::compose(&sigma1, &sigma2);
        let sigma5 = Substitution::compose(&sigma4, &sigma3);
        assert_eq!(mgu(&atom1, &atom2), Some(sigma5));
        assert_eq!(mgu(&atom1, &atom3), Some(sigma4));
        assert_eq!(mgu(&atom1, &atom4), None);
    }

    #[test]
    fn test3(){
        let c1 = Term::Constant(Constant::Number(1.0));
        let c2 = Term::Constant(Constant::Number(2.0));
        let x_ = Variable::of_str("X");
        let y_ = Variable::of_str("Y");
        let x = Term::Variable(x_.clone());
        let y = Term::Variable(y_.clone());
        let a = |x: &Term, y: &Term, z: &Term| Atom{predicate:"a".to_string(), args:vec![x.clone(), y.clone(), z.clone()]};
        let atom1 = a(&c1, &x, &x);
        let atom2 = a(&y, &y, &c1);
        let atom3 = a(&y, &y, &c2);
        let atom4 = a(&x, &y, &y);
        let atom5 = a(&x, &y, &c2);
        let sigma1 = Substitution::substitution(x_.clone(), c1.clone());
        let sigma2 = Substitution::substitution(y_.clone(), c1.clone());
        let sigma12 = Substitution::compose(&sigma1, &sigma2);
        assert_eq!(mgu(&atom1, &atom2), Some(sigma12.clone()));
        assert_eq!(mgu(&atom1, &atom3), None);
        assert_eq!(mgu(&atom1, &atom4), Some(sigma12));
        assert_eq!(mgu(&atom1, &atom5), None);
    }

}