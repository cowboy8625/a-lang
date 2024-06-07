use std::fmt;

pub fn compile_ir_code(
    (ir, symbol_table): (Vec<ir::Instruction>, SymbolTable),
) -> Result<Vec<Instruction>, Vec<String>> {
    todo!("{:#?} {:#?}", ir, symbol_table)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Comment(String),
    Load(Reg, u64),
    Store(Reg, Reg),
    Copy(Reg, Reg),
    Aloc(Reg, Reg),
    Push(Reg),
    Pop(Reg),
    Add(Reg, Reg, Reg),
    Sub(Reg, Reg, Reg),
    Div(Reg, Reg, Reg),
    Mul(Reg, Reg, Reg),
    Inc(Reg),
    Eq(Reg, Reg),
    Jne(Reg, Reg, Label),
    Hult,
    Printreg(Reg),
    Call(Label),
    And(Reg, Reg, Reg),
    Or(Reg, Reg, Reg),
    Shr(Reg, Reg, Reg),
    Return,
    Syscall,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reg {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R30,
    R31,
}
impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reg = &format!("{:?}", self)[1..];
        write!(f, "%{}", reg)
    }
}

pub type Label = String;

fn stringify_instruction_0args(instruction: &str) -> String {
    format!("{:>4}{}", " ", instruction)
}

fn stringify_instruction_1args<A1>(instruction: &str, arg1: &A1) -> String
where
    A1: fmt::Display,
{
    format!("{:>4}{:<10}{}", " ", instruction, arg1)
}

fn stringify_instruction_2args<A1, A2>(instruction: &str, arg1: &A1, arg2: &A2) -> String
where
    A1: fmt::Display,
    A2: fmt::Display,
{
    format!("{:>4}{:<10}{:<10}{}", " ", instruction, arg1, arg2)
}

fn stringify_instruction_3args<A1, A2, A3>(
    instruction: &str,
    arg1: &A1,
    arg2: &A2,
    arg3: &A3,
) -> String
where
    A1: fmt::Display,
    A2: fmt::Display,
    A3: fmt::Display,
{
    format!(
        "{:>4}{:<10}{:<10}{:<10}{}",
        " ", instruction, arg1, arg2, arg3
    )
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let instruction = {
            let i = format!("{:?}", self);
            let end = i.find("(").unwrap();
            i[0..end].to_lowercase()
        };
        let instruction = match self {
            Self::Comment(message) => {
                format!("    ;; {message}")
            }
            Self::Or(arg1, arg2, arg3)
            | Self::Shr(arg1, arg2, arg3)
            | Self::And(arg1, arg2, arg3)
            | Self::Add(arg1, arg2, arg3)
            | Self::Sub(arg1, arg2, arg3)
            | Self::Div(arg1, arg2, arg3)
            | Self::Mul(arg1, arg2, arg3) => {
                stringify_instruction_3args(&instruction, arg1, arg2, arg3)
            }
            Self::Load(arg1, arg2) => stringify_instruction_2args(&instruction, arg1, arg2),
            Self::Eq(arg1, arg2)
            | Self::Store(arg1, arg2)
            | Self::Copy(arg1, arg2)
            | Self::Aloc(arg1, arg2) => stringify_instruction_2args(&instruction, arg1, arg2),
            Self::Printreg(arg1) | Self::Inc(arg1) | Self::Push(arg1) | Self::Pop(arg1) => {
                stringify_instruction_1args(&instruction, arg1)
            }
            Self::Return | Self::Hult | Self::Syscall => stringify_instruction_0args(&instruction),
            _ => panic!("Unknown instruction: {self:?}"),
            // Jne(Reg, Reg, Label),
            // Call(Label),
        };
        writeln!(f, "{instruction}")
    }
}

trait Compile {
    fn compile(&self, symbol_table: &SymbolTable) -> Vec<Instruction>;
}

use crate::{ir, symbol_table::SymbolTable};
impl Compile for ir::Instruction {
    fn compile(&self, st: &SymbolTable) -> Vec<Instruction> {
        match self {
            ir::Instruction::LoadImm(i) => i.compile(st),
            _ => unimplemented!(),
        }
    }
}

impl Compile for ir::LoadImm {
    fn compile(&self, st: &SymbolTable) -> Vec<Instruction> {
        todo!("{st:?}")
    }
}
