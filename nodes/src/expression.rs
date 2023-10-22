use crate::types::types::Type;

pub enum Expression {
    Literal(LiteralExpression),
    Identifier(IdentifierExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Function(FunctionExpression),
    Call(CallExpression),
    Conditional(ConditionalExpression),
    Array(ArrayExpression),
    Object(ObjectExpression),
    Member(MemberExpression),
    Assignment(AssignmentExpression),
}

pub struct LiteralExpression {
    value: String,
    type_: Type,
}

pub struct IdentifierExpression {
    name: String,
    type_: Option<Type>
}

pub struct BinaryExpression {
    left: Box<Expression>,
    operator: BinaryOperator,
    right: Box<Expression>,
}

pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
}

pub struct UnaryExpression {
    operator: UnaryOperator,
    argument: Box<Expression>,
}

pub enum UnaryOperator {
    Minus,
    Increment,
    Decrement,
    Not,
    SizeOf,
}

pub struct FunctionExpression {
    params: Vec<FunctionParameter>,
    body: Box<Expression>,
    return_type: Type,
}

pub struct FunctionParameter {
    name: String,
    type_: Type,
}

pub struct CallExpression {
    callee: Box<Expression>,
    arguments: Vec<Expression>,
}

pub struct ConditionalExpression {
    test: Box<Expression>,
    consequent: Box<Expression>,
    alternate: Box<Expression>,
}

pub struct ArrayExpression {
    elements: Vec<Expression>,
}

pub struct ObjectExpression {
    properties: Vec<Property>,
}

pub struct Property {
    key: String,
    value: Expression,
}

pub struct MemberExpression {
    object: Box<Expression>,
    property: Box<Expression>,
}

pub struct AssignmentExpression {
    operator: AssignmentOperator,
    left: Box<Expression>,
    right: Box<Expression>,
}

pub enum AssignmentOperator {
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
}



