extern crate pest;
extern crate pest_derive;
extern crate lazy_static;

use std::io::{self, BufRead};
use std::io::Write;
use std::fmt;

mod value;
use value::*;
mod heap;
use heap::*;
mod namespacestack;
use namespacestack::*;
mod memory;
use memory::*;

use namespacestack::VecNameSpaceStack as NameSpaceStack;
use heap::VecHeap as Heap;
type HeapAddress = <heap::VecHeap as HeapTrait>::HeapAddress;

mod ast;
use ast::*;
mod parser;
use parser::parse;
mod eval;
use eval::*;


fn prompt() {
    print!("imp # ");
    io::stdout().flush().unwrap();
}

pub fn id_of_instruction(instruction: &Instruction) -> String {
    use Instruction::*;
    match instruction {
        Let { id, .. } => id.to_string(),
        _ => "-".to_string()
    }
}

fn deep_repr(v: &Value, mem: &Memory) -> String {
    match v {
        Value::Integer(i) => format!("{}", i),
        Value::Unit => "()".to_string(),
        Value::Box { heap_address, .. } => {
            let inner = match mem.heap.lookup(heap_address) {
                Ok(v2) => deep_repr(&v2, mem),
                Err(_) => "Ø".to_string()
            };
            format!("@{}({})", heap_address, inner)
        }
    }
}

fn main(){
    let heap_size = 4;
    let mut mem  = Memory::new(heap_size);
    mem.stack.enter_block();
    prompt();
    for line in io::stdin().lock().lines(){
        let input = line.unwrap(); // READ
        if input.starts_with("#") {
            match input.as_str() {
                "#dump_heap" => print!("{}\n", mem.heap),
                _ => print!("unknown directive\n"),
            };
            prompt();
            continue
        }
        match parse(input) {   // PARSE
            Err(msg) => { print!("Parse Error: {}\n", msg) },
            Ok(instr) => {
                match eval_instr(&instr, &mut mem) { // EVAL
                    Err(msg) => { print!("Evaluation Error : {}\n", msg)},
                    Ok(v) => { 
                        print!("{} : {} = {}\n", id_of_instruction(&instr), v.get_type(), deep_repr(&v, &mem));
                    } // PRINT
                }
            }
        };
        prompt()
    }
}