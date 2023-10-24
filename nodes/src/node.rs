use crate::expression::Expression;
use crate::statement::Statement;

pub enum Node {
    Expression(Expression),
    Statement(Statement),
}



