use std::{collections::HashSet, ops::Sub};
use crate::{clause::{Clause, ClauseRightItem}, request::{Request, RequestItem}, 
            atom::Atom, substitution::Substitution, term::Term, unification::mgu, 
            variable::Variable, program::Program, configuration::Configuration};



/*
Return a new clause "c'" that is identical to 'c', except that all variables in c'
are renamed to fresh variables that are not in forbiddenVars.
Works recursively by iterating through all variables in c and checking if they are in forbiddenVars.
The function calls itself recursively on the new clause with the same set.


*/

fn rename(c: &Clause, forbiddenVars: HashSet<Variable>) -> Clause {
    // retourne une clause c' identique à la clause c, à renommage près,
    // de sorte que c' n'a aucune variable dans l'ensemble de variables "interdites"
    for v in c.variables().iter() {
        if forbiddenVars.contains(v) {
            let mut i = 1;
            let mut v2 = v.with_variant(i); // call the Variable function
            while forbiddenVars.contains(&v2) { // if the new variant is in the set btw, we increment by one more variant
                i += 1;
                v2 = v.with_variant(i)
            };
            let subst = &Substitution::substitution(v.clone(), Term::Variable(v2.clone()));
            return rename(&Substitution::apply_to_clause(&subst, &c), forbiddenVars);
        }
    };
    c.clone()
}


fn resolve(goal:&Atom, clause:&Clause) -> Option<(Substitution, Vec<ClauseRightItem>)> {
    // essaie de déduire le but en applicant la clause avec une substitution la plus générale possible
    // renvoie la substitution la plus générale ainsi que les premisses de la clause après application de 
    // la substitution si c'est possible, None sinon
    
    let unifier = mgu(&goal, &clause.goal)?;
    
    let new_caluse = Substitution::apply_to_clause(&unifier, clause);
    let premises = new_caluse.premises.clone();
    
    Some((unifier, premises))
}




fn search_step(mut conf: Configuration, program:&Program) -> Vec<Configuration> {
    // prend une configuration avec une requête contenant au moins un but et un programme
    // fait un pas de recherche à partir de la configuration donnée
    // en tentant de faire une résolution du but en tête de requête (en fin du vec, dans notre représentation)
    // avec la ième clause du programme (si le PC vaut i dans la configuration courante)
    // on renvoie 0, 1, ou 2 configurations (dans un vec), qui déterminent comment continuer la recherche.
    // ce vecteur de configuration est calculé comme suit:
    //  - si le PC a dépassé le nombre de clauses du programme, on renvoie 0 configurations (i.e. vec![])
    //  - si la résolution échoue, on renvoie la configuration initiale avec le PC incrémenté de 1
    //  - sinon on renvoie la configuration initiale avec le PC incrémenté de 1 ainsi que la configuration
    //    obtenue en réécrivant la requête, avec la substitution augmentée du mgu calculé, et un PC à 0
    
    if conf.program_counter >= program.clauses.len() {
        // if the program counter has exceeded the number of clauses in the program, we are done searching
        return vec![];
    }
        
    let clause = rename(&program.clauses[conf.program_counter], conf.request.variables());
    conf.program_counter += 1;
        
    match conf.request.goals.last().unwrap() {
        RequestItem::Atom(goal) => {
            match resolve(&goal, &clause) {
                None => {
                    vec![Configuration { request: conf.request, substitution: conf.substitution, program_counter: conf.program_counter }]
                },
                Some((sigma, mut reqitems)) => {
                    match reqitems.pop().unwrap() {
                        ClauseRightItem::Atom(curr) => {
                            let updated_goal = Substitution::apply_to_atom(&sigma, &curr);
                            let updated_substitution = Substitution::compose(&sigma, &conf.substitution);
                                
                            let updated_request = Request {
                                goals: reqitems.iter().map(|r| {
                                    match r {
                                        ClauseRightItem::Atom(a) => RequestItem::Atom(Substitution::apply_to_atom(&updated_substitution, &a)),
                                        ClauseRightItem::Cut => RequestItem::Cut(0),
                                    }
                                }).collect(),
                            };
                                
                            let mut results = vec![];
                                
                            if updated_request.goals.is_empty() {
                                results.push(Configuration { request: updated_request.clone(), substitution: updated_substitution.clone(), program_counter: 0 });
                            } else {
                                let updated_conf = Configuration { request: updated_request.clone(), substitution: updated_substitution.clone(), program_counter: 0 };
                                results.push(updated_conf);
                            }
                                
                            let updated_clause = Substitution::apply_to_clause(&updated_substitution, &clause);
                            let conf_with_updated_clause = Configuration { request: Request { goals: vec![RequestItem::Atom(updated_goal)] }, substitution: updated_substitution.clone(), program_counter: 0 };
                            results.push(conf_with_updated_clause);
                                
                            if updated_request.goals.is_empty() {
                                results.push(Configuration { request: updated_request.clone(), substitution: updated_substitution.clone(), program_counter: 0 });
                            } else {
                                let conf_with_updated_subst = Configuration { request: updated_request.clone(), substitution: updated_substitution.clone(), program_counter: conf.program_counter };
                                results.push(conf_with_updated_subst);
                            }
                                
                            results
                        },
                        _ => vec![],
                    }
                }
            }
        },
        RequestItem::Cut(_) => vec![],
    }
}
    
    


fn search_next_solution(stack: &mut Vec<Configuration>, program: &Program) 
    -> Option<Substitution> {
    // cherche une solution en partant de la configuration en haut de la pile
    // à chaque petit pas, on empile les configurations produites par le petit pas
    // on s'arrête lorsque 
    // - ou bien la configuration en haut du tas contient une requête vide:
    //   une solution a été trouvée, on la renvoie
    // - ou bien la pile est vide: aucune solution n'a pu être trouvé, on renvoie None
    while let Some(conf) = stack.pop() {
        let res = search_step(conf, program);
        for c in res {
            stack.push(c);
        }
        if let Request { goals, .. } = &stack.last().unwrap().request {
            if goals.is_empty() {
                return Some(stack.last().unwrap().substitution.clone());
            }
        }
    }
    None
}


#[cfg(test)]
mod test {
    use crate::{constant::Constant, variable::Variable, term::Term, atom::Atom};
    use super::*;

    macro_rules! a {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x.clone());
                )*
                Atom {predicate: "a".to_string(), args:temp_vec}
            }
        };
    }

    #[test]
    fn test_rename(){
        let x_: Variable = Variable::of_str("X");
        let y_: Variable  = Variable::of_str("Y");
        let z_: Variable  = Variable::of_str("Z");
        let x : Term = Term::Variable(x_);
        let y : Term = Term::Variable(y_);
        let z : Term = Term::Variable(z_);
        let c1 : Term = Term::Constant(Constant::Number(1.0));
        let c2 : Term = Term::Constant(Constant::Number(2.0));
        let atom1 = a!(x, y, c1);
        let clause1 = Clause{ goal: a!(z, c2, y), premises: vec![ClauseRightItem::Atom(a!(x,y,z,c1,c2))] };
        let clause2 = rename(&clause1, atom1.variables());
        print!("atom1 = {atom1}\nclause1 = {clause1}\nclause2 = {clause2}\n");
        print!("variables communes = {:?}\n", clause2.variables().intersection(&atom1.variables()));
        assert!(clause2.variables().is_disjoint(&atom1.variables()));
    }

    #[test]
    fn test_resolve() {
        let x_: Variable = Variable::of_str("X");
        let y_: Variable  = Variable::of_str("Y");
        let z_: Variable  = Variable::of_str("Z");
        let x : Term = Term::Variable(x_);
        let y : Term = Term::Variable(y_);
        let z : Term = Term::Variable(z_);
        let c1 : Term = Term::Constant(Constant::Number(1.0));
        let c2 : Term = Term::Constant(Constant::Number(2.0));
        let atom1 = a!(x, y, c1);
        let clause1 = Clause{ goal: a!(z, c2, y), premises: vec![ClauseRightItem::Atom(a!(x,y,z,c1,c2)), ClauseRightItem::Atom(a!(c1))] };
        let clause2 = rename(&clause1, atom1.variables());
        let res = resolve(&atom1, &clause2);
        assert_ne!(res, None);
        print!("atom1 = {atom1}\nclause2 = {clause2}\n");
        let (sigma, reqitems) = res.unwrap();
        print!("sigma = {sigma}, attendu [Y_1<-1, Z<-X, Y<-2]\nreqitems = {}, attendu a(X_1, 1, X, 1, 2), a(1)\n", reqitems.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(", "));
        assert_eq!(resolve(&a!(c1), &clause1), None);
        assert_eq!(resolve(&atom1, &clause1), None);        
    }

    #[test]
    fn test_search_step() {
        let x_: Variable = Variable::of_str("X");
        let y_: Variable  = Variable::of_str("Y");
        let z_: Variable  = Variable::of_str("Z");
        let x : Term = Term::Variable(x_.clone());
        let y : Term = Term::Variable(y_.clone());
        let z : Term = Term::Variable(z_.clone());
        let c1 : Term = Term::Constant(Constant::Number(1.0));
        let c2 : Term = Term::Constant(Constant::Number(2.0));
        let atom1 = a!(x, y, c1);
        let clause1 = Clause{ goal: a!(z, c2, y), premises: vec![ClauseRightItem::Atom(a!(x,y,z,c1,c2)), ClauseRightItem::Atom(a!(c1))] };
        let conf1 = Configuration {
            request: Request { goals: vec![RequestItem::Atom(atom1.clone())] },
            substitution: Substitution::substitution(x_.clone(), c1.clone()),
            program_counter: 0,
        };
        let conf2 = Configuration {
            request: Request { goals: vec![RequestItem::Atom(atom1)] },
            substitution: Substitution::substitution(x_, c1.clone()),
            program_counter: 1,
        };
        let prog = Program{ clauses: vec![
            Clause { goal: a!(c1), premises: vec![] },
            clause1
        ]};
        let confs = search_step(conf1, &prog);
        assert_eq!(confs.len(), 1);
        assert_eq!(confs[0], conf2);
        let confs2 = search_step(conf2, &prog);
        assert_eq!(confs2.len(), 2);
        print!("confs2[0] = {}, attendu Conf[:- a(X, Y, 1)., [X<-1], PC=2]\nconfs2[1] = {}, attendu Conf[:- a(X_1, 1, X, 1, 2), a(1)., [X<-1, Y_1<-1, Z<-X, Y<-2]\n", confs2[0], confs2[1])
    }

    #[test]
    fn test_search_next_solution() {
        // CF exercice 2 du TD
        let x_: Variable = Variable::of_str("X");
        let y_: Variable  = Variable::of_str("Y");
        let z_: Variable  = Variable::of_str("Z");
        let x : Term = Term::Variable(x_.clone());
        let y : Term = Term::Variable(y_.clone());
        let z : Term = Term::Variable(z_.clone());
        let c1 : Term = Term::Constant(Constant::Number(1.0));
        let c2 : Term = Term::Constant(Constant::Number(2.0));
        let request = Request { goals: vec![RequestItem::Atom(a!(x,x))] };
        let prog = Program { clauses: vec![
            Clause { goal: a!(c1,c1), premises:vec![]},
            Clause { goal: a!(c2,c1), premises:vec![]},
            Clause { goal: a!(c1,c2), premises:vec![]},
            Clause { goal: a!(c2,x), premises:vec![ClauseRightItem::Atom(a!(x,c1)), ClauseRightItem::Atom(a!(c1,x))]}
        ]};
        let conf0 = Configuration {
            request,
            substitution: Substitution::identity(),
            program_counter: 0
        };
        let mut stack = vec![conf0];
        let first_result = search_next_solution(&mut stack , &prog);
        assert_ne!(first_result, None);
        print!("first_result = {}, attendu [X<-1]\n", first_result.unwrap());
        let second_result = search_next_solution(&mut stack, &prog);
        assert_ne!(second_result, None);
        print!("second_result = {}, attendu [X<-2]\n", second_result.unwrap());
        let third_result = search_next_solution(&mut stack, &prog);
        assert_ne!(third_result, None);
        print!("third_result = {}, attendu [X<-2]\n", third_result.unwrap());
        let none = search_next_solution(&mut stack, &prog);
        assert_eq!(none, None);
    }

}