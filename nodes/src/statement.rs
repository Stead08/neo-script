use crate::expression::Expression;
use crate::types::types::Type;

pub enum Statement {
    VariableBinding(VariableBindingStatement),
    FunctionDeclaration(FunctionDeclarationStatement),
    Return(ReturnStatement),
    If(IfStatement),
    For(ForLoopStatement),
    While(WhileLoopStatement),
    Break,
    Continue,
    Switch(SwitchStatement),
    Throw(ThrowStatement),
    TruCatchFinally(TryCatchFinallyStatement),
}

pub struct VariableBindingStatement {
    name: String,
    type_: Type,
    value: Expression,
}

pub struct FunctionDeclarationStatement {
    name: String,
    args: Vec<(String, Type)>,
    return_type: Type,
    body: Vec<Statement>,
}

pub struct ReturnStatement {
    value: Option<Expression>,
}

pub struct IfStatement {
    test: Expression,
    consequent: Vec<Statement>,
    alternate: Option<Vec<Statement>>,
}

pub struct ForLoopStatement {
    init: Option<Box<Statement>>,
    test: Option<Expression>,
    update: Option<Expression>,
    body: Vec<Statement>,
}

pub struct WhileLoopStatement {
    condition: Expression,
    body: Vec<Statement>,
}

pub struct SwitchStatement {
    discriminant: Expression,
    cases: Vec<SwitchCase>,
}

pub struct SwitchCase {
    test: Option<Expression>,
    consequent: Vec<Statement>,
}

pub struct ThrowStatement {
    value: Expression,
}

pub struct TryCatchFinallyStatement {
    try_body: Vec<Statement>,
    catch_body: Vec<Statement>,
    finally_body: Vec<Statement>,
}



