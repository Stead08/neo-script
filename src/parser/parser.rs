use crate::node::node::Node;
use peg;

peg::parser!(pub grammar neoscript() for str {
    pub rule parse() -> Vec<Node>
        = v:sentences() { v }

    rule sentences() -> Vec<Node>
        = sentence() ** end_of_line()

    rule sentence() -> Node
    = print() / bind_variable() / assignment() / block()

    rule expression() -> Node
    = if_expr() / calc() / for_expr() / print() / bind_variable() / assignment() / _ { Node::None }

    rule block() -> Node
    = "{" _ v:sentences()? _ e:expression()? _ "}" _ {
    let mut nodes = Vec::new();
    if let Some(sentences) = v {
        nodes.extend(sentences);
    }
    if let Some(expr) = e {
        nodes.push(expr);
    }
    Node::Block(nodes)
    }


    rule assignment() -> Node
    = w:word() _ "=" _ v:expression() { Node::Assignment(w, Box::new(v)) }


    rule bind_variable() -> Node
        = "let" _ w:word() _ "=" _ v:expression() ";" { Node::BindVariable(w, Box::new(v)) }


    rule print() -> Node
    = "print" _ v:expression() ";" { Node::DebugPrint(Box::new(v)) }


    rule if_expr() -> Node = "if" _ v:if_cond() { v }

    rule if_cond() -> Node
        = if_elseif() / if_else() / if_only()

    rule if_elseif() -> Node
        = "(" _ cond:expression() _ ")" _ t:block() _ "else if" _ f:if_cond() {
            Node::If(Box::new(cond), vec![t], vec![f])
        }

    rule if_else() -> Node
        = "(" _ cond:expression() _ ")" _ t:block() _ "else" _ f:block() {
            Node::If(Box::new(cond), vec![t], vec![f])
        }

    rule if_only() -> Node
        = "(" _ cond:expression() _ ")" _ t:block() {
            Node::If(Box::new(cond), vec![t], vec![])
        }



    rule for_expr() -> Node
    = "for" _ "(" _ init:assignment() _ ";" _ cond:calc() _ ";" _ update:assignment() _ ")" _ body:block() {
        Node::For(Box::new(init), Box::new(cond), Box::new(update), vec![body]);
        Node::None
    }


    // Calculation rules
    rule calc() -> Node = comp()

    rule comp() -> Node
        = l:expr() _ "==" _ r:comp() { Node::calc('=', l, r) }
        / l:expr() _ "!=" _ r:comp() { Node::calc('!', l, r) }
        / l:expr() _ "<" _ r:comp() { Node::calc('<', l, r) }
        / l:expr() _ ">" _ r:comp() { Node::calc('>', l, r) }
        / l:expr() _ "<=" _ r:comp() { Node::calc('l', l, r) }
        / l:expr() _ ">=" _ r:comp() { Node::calc('g', l, r) }
        / expr()

    rule expr() -> Node
        = l:term() _ "+" _ r:expr() { Node::calc('+', l, r) }
        / l:term() _ "-" _ r:expr() { Node::calc('-', l, r) }
        / term()

    rule term() -> Node
        = l:factor() _ "*" _ r:term() { Node::calc('*', l, r) }
        / l:factor() _ "/" _ r:term() { Node::calc('/', l, r) }
        / l:factor() _ "%" _ r:term() { Node::calc('%', l, r) }
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

    rule end_of_line() = [';' | '\n']+ _
    rule ws() = [' ' | '\n' | '\t']*
    rule lf() = "\n"
    rule line_comment() = "//" (!lf() [_])* lf()
});
