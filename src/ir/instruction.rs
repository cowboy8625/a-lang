use crate::{lexer::Token, parse::Ident};

use super::{Imm, Label, Reg};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Null,
    U64,
}

impl TryFrom<&Ident> for Type {
    type Error = &'static str;
    fn try_from(value: &Ident) -> Result<Self, Self::Error> {
        match value.value().as_str() {
            "u64" => Ok(Self::U64),
            "null" => Ok(Self::Null),
            _ => Err("unknown type"),
        }
    }
}
impl Default for Type {
    fn default() -> Self {
        Self::Null
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    DefFunc(DefFunc),
    LoadImm(LoadImm),
    CopyReg(CopyReg),
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    Grt(Grt),
    Copy(Copy),
    Conditional(Conditional),
    Jump(Jump),
    DefLabel(DefLabel),
    Call(Call),
    Return(Return),
    Enter(Enter),
    Leave(Leave),
}

// impl Instruction {
//     pub fn _is_exit(&self) -> bool {
//         match self {
//             Self::Conditional(..) | Self::Jump(..) => true,
//             _ => false,
//         }
//     }
//
//     pub fn is_enter(&self) -> bool {
//         match self {
//             Self::DefLabel(..) => true,
//             _ => false,
//         }
//     }
// }

macro_rules! from_to {
    ($from:ident, $to:ident) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                Self::$from(value)
            }
        }
    };
}

from_to!(DefFunc, Instruction);
from_to!(LoadImm, Instruction);
from_to!(CopyReg, Instruction);
from_to!(Copy, Instruction);
from_to!(Conditional, Instruction);
from_to!(Jump, Instruction);
from_to!(DefLabel, Instruction);
from_to!(Call, Instruction);
from_to!(Return, Instruction);
from_to!(Enter, Instruction);
from_to!(Leave, Instruction);

macro_rules! op_instruction {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            pub des: Reg,
            pub lhs: Reg,
            pub rhs: Reg,
        }

        impl From<$name> for Instruction {
            fn from(value: $name) -> Self {
                Self::$name(value)
            }
        }
    };
}
op_instruction!(Add);
op_instruction!(Sub);
op_instruction!(Mul);
op_instruction!(Div);
op_instruction!(Grt);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefFunc {
    pub name: String,
    pub params: Vec<(Reg, Type)>,
    pub body: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadImm {
    pub des: Reg,
    pub imm: Imm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopyReg {
    pub des: Reg,
    pub src: Reg,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Copy {
    pub to: Reg,
    pub from: Reg, // Either<Var, Reg>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conditional {
    pub label: Label,
    pub reg: Reg,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub caller: Label,
    pub args: Vec<Reg>,
    pub ret: Reg,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Jump(pub Label);

impl Jump {
    pub fn name(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefLabel(pub Label);

impl DefLabel {
    pub fn name(&self) -> String {
        self.0 .0.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Return(pub Option<Reg>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leave;
