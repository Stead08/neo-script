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
    /** for expression **/
    For(String, i64, i64, Vec<Node>),




}