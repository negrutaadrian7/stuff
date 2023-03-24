use std::fmt::Display;

use crate::{request::Request, substitution::Substitution};

#[derive(Debug, PartialEq)]
pub struct Configuration {
    pub request: Request,
    pub substitution: Substitution,
    pub program_counter: usize
}

impl Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conf[{}, {}, PC={}]", self.request, self.substitution, self.program_counter)
    }
}