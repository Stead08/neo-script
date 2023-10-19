use crate::node::node::Node;
use crate::parser::parser::neoscript;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

struct Context {
    // own variable and value
    vars: HashMap<String, VariableType>,
}

enum VariableType {
    Mutable(i64),
    Immutable(i64),
}

fn run_node(ctx: &mut Context, node: Node) -> Result<i64> {
    match node {
        Node::Number(v) => Ok(v),
        Node::Calc(op, l, r) => {
            let l_val = run_node(ctx, *l)?;
            let r_val = run_node(ctx, *r)?;
            Ok(calc_op(op, l_val, r_val))
        }
        Node::ReferVariable(name) => match ctx.vars.get(&name) {
            Some(VariableType::Immutable(value)) => Ok(*value),
            Some(VariableType::Mutable(value)) => Ok(*value),
            None => Err(anyhow!("Variable '{}' not found", name)),
        },
        Node::BindVariable(name, node) => {
            let val = run_node(ctx, *node)?;
            ctx.vars.insert(name, VariableType::Immutable(val));
            Ok(val)
        }

        Node::BindMutVariable(name, node) => {
            let val = run_node(ctx, *node)?;
            ctx.vars.insert(name, VariableType::Mutable(val));
            Ok(val)
        }
        Node::If(cond, true_n, false_n) => {
            let cond_v = run_node(ctx, *cond)?;
            if cond_v > 0 {
                run_nodes(ctx, &true_n)
            } else {
                run_nodes(ctx, &false_n)
            }
        }
        Node::For(init, cond, update, body) => {
            run_node(ctx, *init)?;
            while {
                let condition_result = run_node(ctx, *cond.clone())?;
                condition_result > 0
            } {
                run_nodes(ctx, &body)?;
                // Update using Assignment
                run_node(ctx, Node::Assignment("i".to_string(), update.clone()))?;
            }
            Ok(0)
        }

        Node::DebugPrint(v) => {
            println!("{}", run_node(ctx, *v)?);
            Ok(0)
        }
        Node::DebugPrintStr(v) => {
            println!("{}", v);
            Ok(0)
        }
        Node::Assignment(var_name, expr) => {
            let is_mut = match ctx.vars.get(&var_name) {
                Some(VariableType::Mutable(_)) => true,
                Some(VariableType::Immutable(_)) => {
                    return Err(anyhow!(
                        "Cannot assign to an immutable variable '{}'",
                        var_name
                    ));
                }
                None => return Err(anyhow!("Variable '{}' not found", var_name)),
            };

            if is_mut {
                let val = run_node(ctx, *expr)?;
                ctx.vars.insert(var_name, VariableType::Mutable(val));
                Ok(val)
            } else {
                Err(anyhow!(
                    "Cannot assign to an immutable variable '{}'",
                    var_name
                ))
            }
        }
        Node::Block(nodes) => run_nodes(ctx, &nodes),
        _ => Err(anyhow!("Unsupported node")),
    }
}

fn calc_op(op: char, l: i64, r: i64) -> i64 {
    match op {
        '+' => l + r,
        '-' => l - r,
        '*' => l * r,
        '/' => l / r,
        '%' => l % r,
        '=' => {
            if l == r {
                1
            } else {
                0
            }
        }
        '!' => {
            if l != r {
                1
            } else {
                0
            }
        }
        '>' => {
            if l > r {
                1
            } else {
                0
            }
        }
        'g' => {
            if l >= r {
                1
            } else {
                0
            }
        }
        '<' => {
            if l < r {
                1
            } else {
                0
            }
        }
        'l' => {
            if l <= r {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}

fn run_nodes(ctx: &mut Context, nodes: &Vec<Node>) -> Result<i64> {
    let mut r = 0;
    for node in nodes {
        r = run_node(ctx, node.clone())?;
    }
    Ok(r)
}

pub fn run(src: &str) -> Result<i64> {
    let nodes = match neoscript::parse(src) {
        Ok(n) => n,
        Err(e) => return Err(anyhow!("Failed to parse source code: {:?}", e)),
    };
    println!("{:?}", nodes);
    let mut ctx = Context {
        vars: HashMap::new(),
    };
    run_nodes(&mut ctx, &nodes)
}
