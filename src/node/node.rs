//JavaScriptの文法要素を表すノードの定義
// ただし全て式として表現できるようにする
#[derive(Debug, Clone)]
pub enum Node {
    /** None **/
    None,
    /** Number(int64) **/
    Number(i64),
    /** Number(float64) **/
    Float(f64),
    /** calc expression **/
    Calc(char, Box<Node>, Box<Node>),
    /** if expression **/
    If(Box<Node>, Vec<Node>, Vec<Node>),
    /** js-like for expression **/
    For(Box<Node>, Box<Node>, Box<Node>, Vec<Node>),
    /** while expression **/
    While(Box<Node>, Vec<Node>),
    /** bind variable expression **/
    BindVariable(String, Box<Node>),
    /** bind mutable variable expression **/
    BindMutVariable(String, Box<Node>),
    /** variable reference expression **/
    ReferVariable(String),
    /** Assignment expression **/
    Assignment(String, Box<Node>),
    /** block expression **/
    Block(Vec<Node>),
    // 開発デバック用
    // Print(console api実装まで）
    DebugPrint(Box<Node>),
    DebugPrintStr(String),
}

impl Node {
    pub fn calc(op: char, left: Node, right: Node) -> Node {
        Node::Calc(op, Box::new(left), Box::new(right))
    }
    pub fn if_expr(cond: Node, t: Vec<Node>, f: Vec<Node>) -> Node {
        Node::If(Box::new(cond), t, f)
    }
}
