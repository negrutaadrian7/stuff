use std::collections::HashMap;
use super::Value as Value;

pub trait NameSpaceStackTrait {
    fn root() -> Self;

    fn get(&self, x: &str) -> Option<&Value>;

    fn add(&mut self, x: String, mutable:bool, v:Value) -> Result<(), String>;

    fn mutate(&mut self, x: String, v:Value) -> Result<(), String>;

    fn enter_block(&mut self) -> ();

    fn exit_block(&mut self) -> Result< (), String>;

}


#[derive(Debug)]
pub struct VecNameSpaceStack { // fields 
    pub stack: Vec<NameSpace>
}

#[derive(Debug)]
pub struct NameSpace {
    pub table: HashMap<String, (bool, Value)>,
}

impl NameSpaceStackTrait for VecNameSpaceStack { // methods

    fn root() -> VecNameSpaceStack {
        VecNameSpaceStack{stack: vec![] }
    }

    fn get(&self, x: &str) -> Option<&Value> {
        let mut i = self.stack.len();
        while i > 0 {
            i = i - 1;
            if self.stack[i].table.contains_key(x) { 
                return Some(&self.stack[i].table.get(x).unwrap().1) 
            };
        };
        None
    }

    fn add(&mut self, x: String, mutable:bool, v:Value) -> Result<(), String> {
        match self.stack.last_mut(){
            None => Err("cannot declare a variable in the root namespace".to_string()),
            Some(ns) => {
                if ns.table.contains_key(&x) { 
                    Err(format!("{} already defined", x))
                } else {
                    ns.table.insert(x, (mutable, v));
                    Ok(())
                }
            }            
        }
    }

    fn mutate(&mut self, x: String, v: Value) -> Result<(), String> {
        let mut i = self.stack.len();
        while i > 0 {
            i = i - 1;
            
            match self.stack[i].table.get_mut(&x) {
                None => continue,
                Some((mutable, value)) => {
                    if ! *mutable { 
                        return Err(format!("{} is not mutable", x)) 
                    };
                    if value.get_type() != v.get_type() { 
                        return Err(format!("type error, variables cannot mutate type"))
                    }
                    *value = v;
                    return Ok(())
                }
            };
        }
        Err(format!("unbound {}", x))
    }

    fn enter_block(&mut self) {
        self.stack.push(NameSpace::new())
    }

    fn exit_block(&mut self) -> Result< (), String> {
        match self.stack.pop() {
            None => Err("exiting root block".to_string()),
            Some(_) => Ok(())
        }
    }

}

impl NameSpace {

    fn new() -> NameSpace {
        NameSpace{ table:HashMap::new() }
    }

}