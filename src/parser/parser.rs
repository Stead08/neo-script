use peg;
use crate::node::node::Node;

peg::parser!(pub grammar neoscript() for str {
    pub rule parse() -> Node
        = v:block() { v }

    rule expression() -> Node
        = print() / if_expr() / for_expr() / bind_variable() / calc() / _ { Node::None }

    rule block() -> Node
    = stmts:statement()* { Node::Block(stmts) }

    rule statement() -> Node
    = bind_variable() / expr_stmt()

    rule expr_stmt() -> Node
    = e:expression() ";" { e }

    rule bind_variable() -> Node
        = "let" _ w:word() _ "=" _ v:expression() ";" { Node::BindVariable(w, Box::new(v)) }

    rule assignment() -> Node
    = w:word() _ "=" _ v:expression() { Node::BindVariable(w, Box::new(v)) }

    rule print() -> Node
        = "print" _ "\"" v:$([^ '"']*) "\"" { Node::DebugPrintStr(v.to_string()) }
        / "print" _ v:calc() { Node::DebugPrint(Box::new(v)) }

    rule if_expr() -> Node
    = "if" _ "(" _ cond:calc() _ ")" _ t:block() elsifs:("else if" _ "(" _ c:calc() _ ")" _ b:block() { (c, b) })* else_block:("else" _ b:block() { b })? {
        let mut nodes = vec![Node::if_expr(cond, vec![t], vec![])];
        for (elsif_cond, elsif_block) in elsifs {
            nodes.push(Node::if_expr(elsif_cond, vec![elsif_block], vec![]));
        }
        if let Some(e_block) = else_block {
            nodes.push(e_block);
        }
        Node::Block(nodes)
    }

    rule for_expr() -> Node
    = "for" _ "(" _ init:assignment() _ ";" _ cond:calc() _ ";" _ update:assignment() _ ")" _ body:block() {
        Node::For(Box::new(init), Box::new(cond), Box::new(update), vec![body]);
        Node::None
    }



    // Calculation rules
    rule calc() -> Node = comp()

    rule comp() -> Node
        = l:expr() "==" _ r:comp() { Node::calc('=', l, r) }
        / l:expr() "!=" _ r:comp() { Node::calc('!', l, r) }
        / l:expr() "<" _ r:comp() { Node::calc('<', l, r) }
        / l:expr() ">" _ r:comp() { Node::calc('>', l, r) }
        / l:expr() "<=" _ r:comp() { Node::calc('l', l, r) }
        / l:expr() ">=" _ r:comp() { Node::calc('g', l, r) }
        / expr()

    rule expr() -> Node
        = l:term() "+" _ r:expr() { Node::calc('+', l, r) }
        / l:term() "-" _ r:expr() { Node::calc('-', l, r) }
        / term()

    rule term() -> Node
        = l:factor() "*" _ r:term() { Node::calc('*', l, r) }
        / l:factor() "/" _ r:term() { Node::calc('/', l, r) }
        / l:factor() "%" _ r:term() { Node::calc('%', l, r) }
        / factor()

    rule factor() -> Node
        = "(" _ v:calc() _ ")" { v }
        / v:number() { Node::Number(v) }
        / v:word() { Node::ReferVariable(v) }

    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule word() -> String
        = v:$(['a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9']*) { v.to_string() }

    rule _()
        = ws() / line_comment()

    rule ws() = [' ' | '\n' | '\t']*
    rule lf() = "\n"
    rule line_comment() = "//" (!lf() [_])* lf()
});
