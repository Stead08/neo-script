use crate::node::node::Node;
use peg;


peg::parser!(pub grammar neoscript() for str {
    pub rule parse() -> Vec<Node>
    = v:first_block() { vec![v] }

    rule first_block() -> Node
    = _ v:sentences()? _ e:expressions()? _ b:blocks()? _ {
    let mut nodes = Vec::new();
    if let Some(sentences) = v {
        nodes.extend(sentences);
    }
    if let Some(expr) = e {
        nodes.extend(expr);
    }
    if let Some(blocks) = b {
        nodes.extend(blocks);
    }
    Node::Block(nodes)
    }

    rule block() -> Node
    = _ "{" _ v:sentences()? _ e:expressions()? _ b:blocks()? _ "}" _ {
    let mut nodes = Vec::new();
    if let Some(sentences) = v {
        nodes.extend(sentences);
    }
    if let Some(expr) = e {
        nodes.extend(expr);
    }
    if let Some(blocks) = b {
        nodes.extend(blocks);
    }
    Node::Block(nodes)
    }

    rule sentence() -> Node
    = s:( if_() / for_()/ while_loop() / print() / bind_variable() / assignment() / block()) _ ";"? _ { s }

    rule expression() -> Node
    = if_() / calc() / for_() /  assignment()


    rule blocks() -> Vec<Node>
        = block() ** _

    rule sentences() -> Vec<Node>
        = sentence() ** _

    rule expressions() -> Vec<Node>
        = expression() ** _


    rule bind_variable() -> Node
        = "const" _ w:word() _ "=" _ v:expression() _ { Node::BindVariable(w, Box::new(v)) }
        / "let" _ w:word() _ "=" _ v:expression() _ { Node::BindMutVariable(w, Box::new(v)) }


    rule print() -> Node
    = "print" _ "\"" v:$([^ '"']*) "\"" {Node::DebugPrintStr(v.to_string())}
    / "print" _ v:expression() ";" { Node::DebugPrint(Box::new(v)) }


    rule if_() -> Node = "if" _ v:if_cond() { v }

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

    rule for_() -> Node
    = "for" _ "(" _ init:for_init() _ ";" _ cond:calc() _ ";" _ update:assignment() _ ";" _ ")" _ body:block() {
        Node::For(Box::new(init), Box::new(cond), Box::new(update), vec![body])
    }

    rule for_init() -> Node
    = w:word() _ "=" _ v:expression() { Node::BindMutVariable(w, Box::new(v)) }

    rule while_loop() -> Node
    = "while" _ "(" _ cond:calc() _ ")" _ "{" _ body:sentences() _ "}" {
        Node::While(Box::new(cond), body)
    }
    // Calculation rules
    rule calc() -> Node = bit()

    rule bit() -> Node
        = l:comp() _ "&" _ r:bit() { Node::calc('&', l, r) }
        / l:comp() _ "|" _ r:bit() { Node::calc('|', l, r) }
        / l:comp() _ "^" _ r:bit() { Node::calc('^', l, r) }
        / comp()

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
        / logic()

    rule logic() -> Node
        = l:term() _ "&&" _ r:term() { Node::calc('&', l, r) }
        / l:term() _ "||" _ r:term() { Node::calc('|', l, r) }
        / l:term() _ "^" _ r:term() { Node::calc('^', l, r) }
        / "!" _ r:term() { Node::calc('!', Node::Number(1), r) }
        / term()

    rule term() -> Node
        = l:factor() _ "*" _ r:term() { Node::calc('*', l, r) }
        / l:factor() _ "/" _ r:term() { Node::calc('/', l, r) }
        / l:factor() _ "%" _ r:term() { Node::calc('%', l, r) }
        / factor()


    rule assignment() -> Node
    = w:word() _ "=" _ v:expression() { Node::Assignment(w, Box::new(v)) }
    / n:word() _ "++" {
    Node::Assignment(
        n.to_string(),
        Box::new(Node::Calc('+', Box::new(Node::ReferVariable(n.to_string())), Box::new(Node::Number(1))))
    )}
    / n:word() _ "--" {
    Node::Assignment(
        n.to_string(),
        Box::new(Node::Calc('-', Box::new(Node::ReferVariable(n.to_string())), Box::new(Node::Number(1))))
    )}
    / n:word() _ "+=" _ v:expression() {
    Node::Assignment(
        n.to_string(),
        Box::new(Node::Calc('+', Box::new(Node::ReferVariable(n.to_string())), Box::new(v)))
    )}
    / n:word() _ "-=" _ v:expression() {
    Node::Assignment(
        n.to_string(),
        Box::new(Node::Calc('-', Box::new(Node::ReferVariable(n.to_string())), Box::new(v)))
    )}
    / n:word() _ "*=" _ v:expression() {
    Node::Assignment(
        n.to_string(),
        Box::new(Node::Calc('*', Box::new(Node::ReferVariable(n.to_string())), Box::new(v)))
    )}
    / n:word() _ "/=" _ v:expression() {
    Node::Assignment(
        n.to_string(),
        Box::new(Node::Calc('/', Box::new(Node::ReferVariable(n.to_string())), Box::new(v)))
    )}
    / n:word() _ "%=" _ v:expression() {
    Node::Assignment(
        n.to_string(),
        Box::new(Node::Calc('%', Box::new(Node::ReferVariable(n.to_string())), Box::new(v)))
    )
    }
    / factor()

    rule factor() -> Node
        = "(" _ v:calc() _ ")" { v }
        / v:boolean() { Node::Boolean(v) }
        / v:number() { Node::Number(v) }
        / v:word() { Node::ReferVariable(v) }

    rule boolean() -> bool
        = "true" { true }
        / "false" { false }

    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule word() -> String
        = v:$(['a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9']*) { v.to_string() }

    rule _
        = line_comment() / ws()

    rule ws() = [' ' | '\n' | '\t']*
    rule lf() = "\n"
    rule line_comment() = "//" _ (!lf() [_])* lf()?
});

