use crate::{lexer::Token, parse::Ident};

use super::{Imm, Label, Reg};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Null,
    U64,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::U64 => write!(f, "u64"),
        }
    }
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

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DefFunc(i) => write!(f, "{i}"),
            Self::LoadImm(i) => write!(f, "{i}"),
            Self::CopyReg(i) => write!(f, "{i}"),
            Self::Add(i) => write!(f, "{i}"),
            Self::Sub(i) => write!(f, "{i}"),
            Self::Mul(i) => write!(f, "{i}"),
            Self::Div(i) => write!(f, "{i}"),
            Self::Grt(i) => write!(f, "{i}"),
            Self::Copy(i) => write!(f, "{i}"),
            Self::Conditional(i) => write!(f, "{i}"),
            Self::Jump(i) => write!(f, "{i}"),
            Self::DefLabel(i) => write!(f, "{i}"),
            Self::Call(i) => write!(f, "{i}"),
            Self::Return(i) => write!(f, "{i}"),
            Self::Enter(i) => write!(f, "{i}"),
            Self::Leave(i) => write!(f, "{i}"),
        }
    }
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

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let op = match stringify!($name) {
                    "Add" => "+",
                    "Sub" => "-",
                    "Mul" => "*",
                    "Div" => "/",
                    "Grt" => ">",
                    _ => unreachable!(),
                };
                write!(f, "    {} = {} {} {}", self.des, self.lhs, op, self.rhs)
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

impl std::fmt::Display for DefFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self
            .params
            .iter()
            .map(|(r, t)| format!("{r}: {t}"))
            .collect::<Vec<_>>()
            .join(", ");
        let body = self
            .body
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "function {}({}) {{\n{}\n}}\n\n", self.name, params, body)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadImm {
    pub des: Reg,
    pub imm: Imm,
}

impl std::fmt::Display for LoadImm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    load {} {}", self.des, self.imm)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopyReg {
    pub des: Reg,
    pub src: Reg,
}

impl std::fmt::Display for CopyReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    copyreg {} {}", self.des, self.src)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Copy {
    pub to: Reg,
    pub from: Reg, // Either<Var, Reg>,
}

impl std::fmt::Display for Copy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    copy {} {}", self.to, self.from)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conditional {
    pub label: Label,
    pub reg: Reg,
}

impl std::fmt::Display for Conditional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    if {} goto {}", self.reg, self.label)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub caller: Label,
    pub args: Vec<Reg>,
    pub ret: Reg,
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = self
            .args
            .iter()
            .map(|r| format!("{r}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "    call {}({}) -> {}", self.caller, args, self.ret)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Jump(pub Label);

impl std::fmt::Display for Jump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    goto {}", self.0)
    }
}

impl Jump {
    pub fn name(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefLabel(pub Label);

impl std::fmt::Display for DefLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.0)
    }
}

impl DefLabel {
    pub fn name(&self) -> String {
        self.0 .0.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Return(pub Option<Reg>);

impl std::fmt::Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "    return {}", self.0)
        match &self.0 {
            Some(reg) => write!(f, "    return {}", reg),
            None => write!(f, "    return"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enter;

impl std::fmt::Display for Enter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    enter")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leave;

impl std::fmt::Display for Leave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    leave")
    }
}
