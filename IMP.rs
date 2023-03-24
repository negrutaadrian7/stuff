use std::fmt;




//////////////////////////////////////////////////////////
// Arbre de syntaxe abstraite
//////////////////////////////////////////////////////////

/*
If we don't know the size of the Vehicle for exemple, we can't use the stack, so one solution is to do this on the heap, and the stack will point the that value that is situated
on the heap


*/


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Integer(i32),
    Boolean(bool),
    Unit
}

use Value::*;

impl fmt::Display for Value {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Unit => write!(f, "()")
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Const(Value),
    Var(String),
    BinOp(Box<Expression>, String, Box<Expression>),
}

use Expression::*;

#[derive(Debug)]
pub enum Instruction {
    Expr(Expression), //Const; Var; BinOp(recursive(Const or Var), String, recursive(Const or Var))
    IfElse{
        cond: Expression,
        cond_true: Box<Instruction>,
        cond_false: Box<Instruction>,
    },
    Let(String, Expression),
    LetMut(String, Expression),
    Mutate(String, Expression),
    While(Expression, Box<Instruction>),
    Block(Vec<Instruction>)
}

use Instruction::*;

//////////////////////////////////////////////////////////
// Espaces de noms
//////////////////////////////////////////////////////////

use std::collections::HashMap;

pub trait NameSpace { // l'interface 
    fn root() -> Self;

    fn get(&self, x: &str) -> Option<&Value>;

    fn add(&mut self, x: String, _v:Value) -> Result<(), String>;

    fn enter_block(&mut self) -> ();

    fn exit_block(&mut self) -> Result< (), String>;

}


pub struct VNameSpace {
    stack: Vec<HashMap<String, Value>>
}

// implementation pour la classe VNameSpace de l'interface NameSpace

impl NameSpace for VNameSpace{ // implement the interface for VNameSpace class 
    
    fn root() -> VNameSpace {
        VNameSpace { 
            stack: vec! [HashMap::new()],
        }
    }

    fn get(&self, x: &str) -> Option<&Value> {
        for element in self.stack.iter().rev(){ // start from the first element, (adding an element will shift all the elements to the right, that's why I used the rev function in addition)
            if let Some(v) = element.get(x){
                return Some(v);
            }
        }
        None
    }

    fn add(&mut self, x: String, v:Value) -> Result<(), String> {
        if self.stack.last_mut().unwrap().contains_key(&x){
            return Err(format!("Variable {} already exists in this scope", x));
        }
        self.stack.last_mut().unwrap().insert(x, v);
        Ok(())
    }


    fn enter_block(&mut self) -> () { 
        self.stack.push(HashMap::new()); // on rentre dans un block
    }


    fn exit_block(&mut self) -> Result< (), String> { // sort d'un bloc, donc changement de namespace
        if self.stack.len() == 1{
            return Err("Can't exit the root block".to_string());
        }
        self.stack.pop();
        Ok(())
    }

}



//////////////////////////////////////////////////////////
// Fonctions d'évaluation
//////////////////////////////////////////////////////////

#[allow(dead_code)]
fn eval_expr<NS: NameSpace>(expr: &Expression, ns: &NS) -> Result<Value, String> { // return the value of the operation
    match expr {
        Const(c) => Ok(*c),
        Var(x) => {
            if let Some(value) = ns.get(&*x) {
                Ok(*value)
            }
            else {
                Err("Variable innexistante".to_string())
            }
        },
        

        BinOp(left, op, right) => { // operation binaire
            let l = eval_expr(left, ns)?;
            let r = eval_expr(right, ns)?;
            
            match (l, op.as_ref(), r) {
                (Integer(nb1), "+", Integer(nb2)) => Ok(Integer(nb1 + nb2)),
                (Integer(nb1), "-", Integer(nb2)) => Ok(Integer(nb1 - nb2)),
                (Integer(nb1), "*", Integer(nb2)) => Ok(Integer(nb1 * nb2)),
                (Integer(nb1), "/", Integer(nb2)) => { // division
                    if nb2 == 0 {
                        return Err("division by zero".to_string());
                    }
                    Ok(Integer(nb1 / nb2))
                },
                
                (Integer(nb1), "%", Integer(nb2)) => {
                    if nb2 == 0 {
                        return Err("division by zero".to_string());
                    }
                    Ok(Value::Integer(nb1 % nb2)) 
                },
                
                (Boolean(nb1), "&&", Boolean(nb2)) => Ok(Boolean(nb1 && nb2)),
                (Boolean(nb1), "||", Boolean(nb2)) => Ok(Boolean(nb1 || nb2)),
                (Integer(nb1), "<", Integer(nb2)) => Ok(Boolean(nb1 < nb2)),
                (Integer(nb1), ">", Integer(nb2)) => Ok(Boolean(nb1 > nb2)),
                (Integer(nb1), "<=", Integer(nb2)) => Ok(Boolean(nb1 <= nb2)),
                (Integer(nb1), ">=", Integer(nb2)) => Ok(Boolean(nb1 >= nb2)),
                (Integer(nb1), "==", Integer(nb2)) => Ok(Boolean(nb1 == nb2)),
                (Integer(nb1), "!=", Integer(nb2)) => Ok(Boolean(nb1 != nb2)),
                (Boolean(nb1), "==", Boolean(nb2)) => Ok(Boolean(nb1 == nb2)),
                (Boolean(nb1), "!=", Boolean(nb2)) => Ok(Boolean(nb1 != nb2)),
                
                _ => Err(format!("Incompatible operands for binary operator: {:?} {:?} {:?}", l, op, r)),
            }
        }
    }
}


#[allow(dead_code)]
fn eval<NS:NameSpace>(instruction: &Instruction, ns: &mut NS) -> Result<Value, String> {
    match instruction {
        Expr(expr) => {eval_expr(&expr, ns)},
        IfElse{cond, cond_true, cond_false} => {
            let cond_val = eval_expr(cond, ns)?;
            match cond_val {
                Boolean(b) => {
                    if b {
                        eval(cond_true, ns)
                    }
                    else{
                        eval(cond_false,ns)
                    }
                },
                _ => return Err(format!("If-else condition need to be a boolean, instead {:?}", cond_val))
            }
        },
        
        
        Let(x, e) => {
            let v = eval_expr(e, ns)?;
            ns.add(x.to_owned(), v)?;
            Ok(Unit)
        },
        
        
        LetMut(x, e) => {
            let v = eval_expr(e, ns)?;
            ns.add(x.to_owned(), v)?;
            Ok(Unit)
        },
        
        
        Mutate(x, e) => {
            let v = eval_expr(e, ns)?;
            ns.add(x.to_owned(), v)?;
            Ok(Unit)
        },
        
        
        While(cond, body) => {
            let mut res = Unit;
            while let Boolean(true) = eval_expr(cond, ns)? {
                res = eval(body, ns)?;
            }
            Ok(res)
        },
        
        
        Block(instructions) => {
            ns.enter_block();
            let mut res = Value::Unit;
            for i in instructions{
                res = eval(i, ns)?;
            }
            ns.exit_block()?;
            Ok(res)
        },
    }
}
/*
On cherche les variables dans le dernier element de notre stack, puisque si on sort d'un namespace on fait un pop.
*/




// Main function 



//////////////////////////////////////////////////////////
// Tests
//////////////////////////////////////////////////////////

fn constant(v:Value) -> Box<Expression> { 
    Box::new(Const(v))
}

#[test]
fn test1_1() -> Result<(),String> {
    let mut ns = VNameSpace::root();
    ns.enter_block();
    let instr = Expr(BinOp(constant(Integer(1)),"+".to_string(),constant(Integer(1))));
    let value = Integer(2);
    assert_eq!(eval(&instr, &mut ns)?,value);
    Ok(())
}