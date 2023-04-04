use std::fmt::Display;

use crate::{request::Request, substitution::Substitution};

#[derive(Debug, PartialEq)]
pub struct Configuration {
    pub request: Request, // la requete qu'on cherche a reecrire
    pub substitution: Substitution, // la substitution obtenue par composition de tous les unificateurs le long de la branche de l'arbre
    pub program_counter: usize // a partir de quelle clause du programme il faut continuer l'exploration des diferentes clauses qui permettent de faire un pas de reecriture
}

impl Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conf[{}, {}, PC={}]", self.request, self.substitution, self.program_counter)
    }
}