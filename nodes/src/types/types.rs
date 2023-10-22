
use crate::types::array::Array;
use crate::types::bool::Boolean;
use crate::types::char::Character;
use crate::types::float::Float;
use crate::types::int::Integer;
use crate::types::never::Never;
use crate::types::string::String_;
use crate::types::tuple::Tuple;
use crate::types::unit::Unit;

/// The type of a node.
pub enum Type {
    Int(Integer),
    Float(Float),
    Char(Character),
    String(String_),
    Bool(Boolean),
    Array(Array),
    Tuple(Tuple),
    Unit(Unit),
    Never(Never),
    Null,
}