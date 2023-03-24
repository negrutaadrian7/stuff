use super::*;

fn eval_lexpr(lexpr: &LeftExpression, mem: &Memory) -> Result<Value, String> {
    use LeftExpression::*;
    use Value::*;
    match lexpr {
        Identifier(id) => match mem.stack.get(id) {
                Some(v) => Ok(v.clone()),
                None => Err(format!("unbound {}",id)),
            },
        Star(lexpr2) => {
            let v =  eval_lexpr(lexpr2, mem)?;
            match v {
                Box { heap_address, .. } => mem.heap.lookup(&heap_address),
                _ => Err(format!("type error: {} has type {}, Box expected", lexpr2, v.get_type()))
            }
        }
    }
}


fn eval_expr(expr: &Expression, mem: &mut Memory) -> Result<Value, String> {
    use Expression::*;
    match expr {
        Const(v) => Ok(Value::Integer(*v)),
        BinOp(fg, op, fd) => {
            match [eval_expr(fg, mem)?, eval_expr(fd, mem)?] {
                [Value::Integer(ig), Value::Integer(id)] => match *op {
                    "+" => Ok(Value::Integer(ig+id)),
                    "-" => Ok(Value::Integer(ig-id)),
                    "*" => Ok(Value::Integer(ig*id)),
                    "/" => Ok(Value::Integer(ig/id)),
                    "%" => Ok(Value::Integer(ig % id)),
                    _ => unreachable!()
                }
                _ => Err(format!("type error, int expected in {:?}", &expr))
            }
        },
        ValueAt(lexpr) => eval_lexpr(lexpr, mem),
        NewBox(expr2) => {
            let v = eval_expr(expr2, mem)?;
            let heap_address = mem.heap.allocate_box(v.clone())?;
            Ok(Value::Box{ heap_address, box_type: v.get_type()})
        }
    }
}

pub fn eval_instr(instruction: &Instruction, mem: &mut Memory) -> Result<Value, String> {
    use Instruction::*;
    match instruction {
        Expr(e) => eval_expr(e, mem),
        Let { id, mutable, expr } => {
            let v = eval_expr(expr, mem)?;
            mem.stack.add(id.to_string(),*mutable, v.clone())?;
            Ok(v)
        }
        Block(instrs) => {
            let mut res = Value::Unit;
            mem.stack.enter_block();
            for instr in instrs {
                res = eval_instr(instr, mem)?;
            };
            mem.stack.exit_block()?;
            Ok(res)
        },
        WriteAt(lexpr, expr) => {
            match lexpr {
                LeftExpression::Identifier(id) => {
                    let v = eval_expr(expr, mem)?;
                    mem.stack.mutate(id.to_string(), v)?;
                    Ok(Value::Unit)
                },
                LeftExpression::Star(lexpr2) => {
                    let v = eval_lexpr(lexpr2, mem)?;
                    match v {
                        Value::Box{heap_address,..} => {
                            let v2 = eval_expr(expr, mem)?;
                            mem.heap.update(&heap_address, v2)?;
                            Ok(Value::Unit)
                        },
                        _ => Err(format!("type error: Box<..> type expected for {}", lexpr)) 
                    }
                }
            }
        },
        Free(lexpr) => {
            match eval_lexpr(lexpr, mem)? {
                Value::Box { heap_address, .. } => {
                    mem.heap.free(&heap_address)?;
                    Ok(Value::Unit)
                },
                _ => Err(format!("type error: Box<..> type expected for {}", lexpr)) 
            }
        },
        Drop(lexpr) => {
            match eval_lexpr(lexpr, mem)? {
                Value::Box { heap_address, .. } => {
                    mem.heap.drop(&heap_address)?;
                    Ok(Value::Unit)
                },
                _ => Err(format!("type error: Box<..> type expected for {}", lexpr)) 
            }
        },
    }
}
