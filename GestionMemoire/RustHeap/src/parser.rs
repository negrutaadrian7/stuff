// see https://pest.rs/book/ and https://pest.rs/book/examples/calculator.html
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct MyParser;

use super::*;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(ADD, Left) | Op::infix(SUBTRACT, Left))
            .op(Op::infix(MULTIPLY, Left) | Op::infix(DIVIDE, Left) | Op::infix(MODULO, Left))
//            .op(Op::prefix(unary_minus))
    };
}

fn parse_lexpr(mut pairs: Pairs<Rule>) -> LeftExpression {
    let first_rule = pairs.next().unwrap();
    match first_rule.as_rule() {
        Rule::identifier => LeftExpression::Identifier(first_rule.as_str().to_string()),
        Rule::deref => LeftExpression::Star(Box::new(parse_lexpr(first_rule.into_inner()))),
        Rule::lexpr => parse_lexpr(first_rule.into_inner()),
        _ => unreachable!()
    }
}


fn parse_expr(pairs: Pairs<Rule>) -> Expression {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expression::Const(primary.as_str().parse::<i32>().unwrap()),
            Rule::lexpr => Expression::ValueAt(parse_lexpr(primary.into_inner())),
            Rule::expr => parse_expr(primary.into_inner()),
            Rule::boxexpr => Expression::NewBox(Box::new(parse_expr(primary.into_inner()))),
            rule => unreachable!("parse_expr expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op2 = match op.as_span().as_str() {
                "+" => "+",
                "-" => "-",
                "*" => "*",
                "/" => "/",
                "%" => "%",
                "==" => "==",
                ">=" => ">=",
                "<=" => "<=",
                ">" => ">",
                "<" => "<",
                "&&" => "&&",
                "||" => "||",
                _ => unreachable!()
            };        
            Expression::BinOp(Box::new(lhs), op2, Box::new(rhs))
        })
        .parse(pairs)
}

fn parse_block(pairs: &mut Pairs<Rule>) -> Result<Vec<Instruction>, String> {
    let first_rule = pairs.next().unwrap();
    let mut res = vec![];
    match first_rule.as_rule() {
        Rule::empty_block => { },
        Rule::non_empty_block => {
            let mut rules = first_rule.into_inner();
            while let Some(rule) = rules.next(){
                if rule.as_rule() == Rule::instr {
                    let instr = parse_instr(&mut rule.into_inner())?;
                    res.push(instr)
                }
            }
        },
        _ => unreachable!("parse_block expected instrs, found {:?}", first_rule),
    };
    Ok(res)
}

fn parse_instr(pairs: &mut Pairs<Rule>) -> Result<Instruction, String> {
    let first_rule = pairs.next().unwrap();
    match first_rule.as_rule() {
        Rule::expr => Ok(Instruction::Expr(parse_expr(first_rule.into_inner()))),
        Rule::let_equals => {
            let mut rules = first_rule.into_inner();
            let id = rules.next().unwrap().as_span().as_str().to_string();
            let expr = rules.next().unwrap().into_inner();
            Ok(Instruction::Let{id, mutable: false, expr: parse_expr(expr) })
        },
        Rule::let_mut_equals => {
            let mut rules = first_rule.into_inner();
            let id = rules.next().unwrap().as_span().as_str().to_string();
            let expr = rules.next().unwrap().into_inner();
            Ok(Instruction::Let{id, mutable: true, expr: parse_expr(expr) })
        },
        Rule::update_instr => {
            let mut rules = first_rule.into_inner();
            let lexpr = parse_lexpr(rules.next().unwrap().into_inner());
            let expr = parse_expr(rules.next().unwrap().into_inner());
            Ok(Instruction::WriteAt(lexpr, expr))
        }
        Rule::instrs => {
            Ok(Instruction::Block(parse_block(&mut first_rule.into_inner())?))
        },
        Rule::free_instr => {
            let lexpr = parse_lexpr(first_rule.into_inner());
            Ok(Instruction::Free(lexpr))
        },
        Rule::drop_instr => {
            let lexpr = parse_lexpr(first_rule.into_inner());
            Ok(Instruction::Drop(lexpr))
        },
        _ => unreachable!("parse_instr expected instr, found {:?}", first_rule),
    }
}

pub fn parse(input: String) -> Result<Instruction, String> {
    match MyParser::parse(Rule::start_rule, &input) {
        Ok(mut pairs) => {
            let first_rule = pairs.next().unwrap();
            match first_rule.as_rule() {
                Rule::instr => {
                    parse_instr(&mut first_rule.into_inner())
                }
                _ => { panic!("the grammar is not as expected") }
            }                
        },
        Err(e) => { Err(format!("{:?}", e)) }
    }
}