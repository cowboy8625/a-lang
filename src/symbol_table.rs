// -------------------------------------------------------------
// | Symbol     | Type       | Data Type | Scope    | Location |
// -------------------------------------------------------------
// | add        | function   | i32 -> i32| global   | L1       |
// | x          | parameter  | i32       | add      | L1       |
// | y          | parameter  | i32       | add      | L1       |
// | two        | function   | i32 -> i32| global   | L4       |
// | x          | parameter  | i32       | two      | L4       |
// | one        | function   | i32 -> i32| global   | L7       |
// | x          | parameter  | i32       | one      | L7       |
// | main       | function   | () -> i32 | global   | L10      |
// | x          | variable   | i32       | main     | L10      |
// -------------------------------------------------------------
// fn add(x: i32, y: i32): i32  {
//     return x + y;
// }
//
// fn two(x: i32): i32 {
//     return add(x, 2);
// }
//
// fn one(x: i32): i32 {
//     return two(x);
// }
//
// fn main(): i32 {
//     let x: i32 = 100;
//     return one(x);
// }

use crate::ir::Reg;
use crate::lexer::Span;
use std::collections::HashMap;
pub type SymbolTable = HashMap<Symbol, SymbolData>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolType {
    Function, // Return type
    Variable, // Variable type
    // Constant, // Constant type
    /// variable names created inside of a function definition
    Parameter, // Parameter type
               // / variable names created inside of a for loop
               // Control, // Control type
               // Structure, // Name of a structure is Cutsom type
               // Enumeration, // Name of an enumeration is Cutsom type
               // Macro, // Macro type?  .......
               // / varibale used in a `use` statement
               // Module, // Module type? ......
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scope {
    Global,
    Function(String),
}

impl Default for Scope {
    fn default() -> Self {
        Self::Global
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub scope: Scope,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeName {
    Bool,
    Char,
    Custom(String),
    F32,
    F64,
    I32,
    I64,
    Null,
    Str,
    U32,
    U64,
    Void,
}

impl<T> From<T> for TypeName
where
    T: std::fmt::Display,
{
    fn from(value: T) -> Self {
        match value.to_string().as_str() {
            "null" => TypeName::Null,
            "i32" => TypeName::I32,
            "i64" => TypeName::I64,
            "f32" => TypeName::F32,
            "f64" => TypeName::F64,
            "u32" => TypeName::U32,
            "u64" => TypeName::U64,
            "void" => TypeName::Void,
            "bool" => TypeName::Bool,
            "char" => TypeName::Char,
            "str" => TypeName::Str,
            ty => TypeName::Custom(ty.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolData {
    pub ty: SymbolType,
    pub scope: Scope,
    pub type_name: TypeName,
    pub reg: Option<Reg>,
    pub span: Span,
}
