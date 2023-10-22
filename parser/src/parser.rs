use nodes::node::Node;
use peg;


peg::parser!(pub grammar neoscript() for str {
    pub rule parse() -> Vec<Node>
    = todo!()


    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule word() -> String
        = v:$(['a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9']*) { v.to_string() }

    rule string() -> String
        = "\"" v:$([^ '"']*) "\"" { v.to_string() }

    rule _
        = line_comment() / ws()

    rule ws() = [' ' | '\n' | '\t']*
    rule lf() = "\n"
    rule line_comment() = "//" _ (!lf() [_])* lf()?
});

