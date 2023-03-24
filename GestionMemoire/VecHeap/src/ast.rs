use std::fmt::Display;

use super::*;

#[derive(Debug)]
pub enum Expression {
    Const(i32),
    NewBox(Box<Expression>),
    ValueAt(LeftExpression),
    BinOp(Box<Expression>, &'static str, Box<Expression>),
}

#[derive(Debug)]
pub enum LeftExpression {
    Identifier(String),
    Star(Box<LeftExpression>), 
    /*  
    Dereference operation, which is denoted by the '*' operator. 
    Box<LeftExpression> contains a boxed pointer to another LeftExpression value, which represents the value being dereferenced.
    */
}

impl Display for LeftExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LeftExpression::*;
        match self {
            Identifier(id) => {write!(f, "{}", id)? },
            Star(lexpr) => {write!(f, "*{}", lexpr)?},
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum Instruction {
    Expr(Expression),
    Let{id:String, mutable:bool, expr:Expression},
    Block(Vec<Instruction>),
    WriteAt(LeftExpression, Expression),
    Free(LeftExpression),
    Drop(LeftExpression)
} 