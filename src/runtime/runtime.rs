use std::collections::HashMap;
use anyhow::{Result, anyhow};
use crate::parser::parser::neoscript;
use crate::node::node::Node;

struct Context {
    // own variable and value
    vars: HashMap<String, i64>
}

fn run_node(ctx: &mut Context, node: Node) -> Result<i64> {
    match node {
        Node::Number(v) => Ok(v),
        Node::Calc(op, l, r) => {
            let l_val = run_node(ctx, *l)?;
            let r_val = run_node(ctx, *r)?;
            Ok(calc_op(op, l_val, r_val))
        },
        Node::ReferVariable(name) => {
            ctx.vars.get(&name).cloned().ok_or(anyhow!("Variable '{}' not found", name))
        },
        Node::BindVariable(name, node) => {
            let val = run_node(ctx, *node)?;
            ctx.vars.insert(name, val);
            Ok(val)
        },
        Node::If(cond, true_n, false_n) => {
            let cond_v = run_node(ctx, *cond)?;
            if cond_v > 0 {
                run_nodes(ctx, &true_n)
            } else {
                run_nodes(ctx, &false_n)
            }
        },
        Node::For(init, cond, update, body) => {
            run_node(ctx, *init)?;

            while run_node(ctx, *cond.clone())? != 0 {
                for stmt in body.iter() {
                    run_node(ctx, stmt.clone())?;
                }
                run_node(ctx, *update.clone())?;
            }

            Ok(0)  // for式は値を返さないと仮定
        },
        Node::DebugPrint(v) => {
            println!("{:?}", run_node(ctx, *v)?);
            Ok(0)
        },
        Node::DebugPrintStr(v) => {
            println!("{}", v);
            Ok(0)
        },
        Node::Block(nodes) => {
            run_nodes(ctx, &nodes)
        },
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
        '=' => if l == r { 1 } else { 0 },
        '!' => if l != r { 1 } else { 0 },
        '>' => if l > r { 1 } else { 0 },
        'g' => if l >= r { 1 } else { 0 },
        '<' => if l < r { 1 } else { 0 },
        'l' => if l <= r { 1 } else { 0 },
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

pub fn run(src: &str) {
    let nodes = neoscript::parse(src).unwrap();
    let mut ctx = Context { vars: HashMap::new() };
    let _ = run_nodes(&mut ctx, &vec![nodes]);
}
