mod reg_state;
#[cfg(test)]
mod test;
pub mod x86reg;
use reg_state::RegState;
pub use std::fmt;
pub use x86reg::*;

use crate::{ir, symbol_table::SymbolTable};
// pub fn code_gen(ir: Vec<ir::Instruction>) -> Result<String, Vec<String>> {
//     compile_ir_code(ir).and_then(instruction_to_string)
// }

pub fn compile_ir_code(
    (ir, symbol_table): (Vec<ir::Instruction>, SymbolTable),
) -> Result<Vec<Instruction>, Vec<String>> {
    let mut state = RegState::default();
    Ok(ir
        .iter()
        .map(|i| i.compile(&mut state, &symbol_table))
        .collect::<Vec<Vec<Instruction>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<Instruction>>())
}

pub fn instruction_to_string(ir: Vec<Instruction>) -> Result<String, Vec<String>> {
    Ok(ir.iter().map(ToString::to_string).collect())
}

trait Compile {
    fn compile(&self, state: &mut RegState, symbol_table: &SymbolTable) -> Vec<Instruction>;
}

trait TypeSize {
    fn size(&self) -> &'static str;
}

impl TypeSize for ir::Type {
    fn size(&self) -> &'static str {
        match self {
            Self::U64 => "qword",
            Self::Null => unreachable!("no size"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Comment(String),
    MoveImm(X86Reg, u64),
    MoveReg(X86Reg, X86Reg),
    MoveMemReg(Mem, X86Reg),
    MoveRegMem(X86Reg, Mem),
    MoveZx(X86Reg),
    Add(X86Reg, X86Reg),
    Sub(X86Reg, X86Reg),
    Mul(X86Reg, X86Reg),
    Div(X86Reg, X86Reg),
    DefLabel(String),
    Call(String),
    Jump(String),
    JumpZero(String),
    Cmp(X86Reg, X86Reg),
    Test(X86Reg, X86Reg),
    SetG,
    ProLog,
    Epilog,
    Syscall,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Comment(message) => {
                writeln!(f, "    ;; {message}")
            }
            Self::MoveImm(des, value) => {
                writeln!(
                    f,
                    "{:>4}{:<10}{:<10}{}",
                    " ",
                    "mov",
                    format!("{des},"),
                    value
                )
            }
            Self::MoveReg(des, src) => {
                writeln!(f, "{:>4}{:<10}{:<10}{}", " ", "mov", format!("{des},"), src)
            }
            Self::MoveMemReg(mem, reg) => {
                let fmt_mem = format!("{mem},");
                writeln!(f, "{:>4}{:<10}{:<10}{}", " ", "mov", fmt_mem, reg)
            }
            Self::MoveRegMem(reg, mem) => {
                writeln!(f, "{:>4}{:<10}{:<16}{}", " ", "mov", format!("{reg},"), mem,)
            }
            Self::MoveZx(src) => {
                writeln!(f, "{:>4}{:<10}{:<10}al", " ", "movzx", format!("{src},"),)
            }
            Self::Add(des, reg) => {
                writeln!(f, "{:>4}{:<10}{:<10}{}", " ", "add", format!("{des},"), reg)
            }
            Self::Sub(des, reg) => {
                writeln!(f, "{:>4}{:<10}{:<10}{}", " ", "sub", format!("{des},"), reg)
            }
            Self::Mul(des, reg) => writeln!(
                f,
                "{:>4}{:<10}{:<10}{}",
                " ",
                "imul",
                format!("{des},"),
                reg
            ),
            Self::Div(des, reg) => writeln!(
                f,
                "{:>4}{:<10}{:<10}{}",
                " ",
                "idiv",
                format!("{des},"),
                reg
            ),
            Self::DefLabel(name) => writeln!(f, "{name}__:"),
            Self::Call(name) => writeln!(f, "{:>4}{:<10}{name}__", " ", "call"),
            Self::Jump(name) => writeln!(f, "{:>4}{:<10}{name}__", " ", "jmp"),
            Self::JumpZero(name) => writeln!(f, "{:>4}{:<10}{name}__", " ", "jz"),
            Self::Cmp(lhs, rhs) => {
                writeln!(f, "{:>4}{:<10}{:<10}{}", " ", "cmp", format!("{lhs},"), rhs)
            }
            Self::Test(lhs, rhs) => writeln!(
                f,
                "{:>4}{:<10}{:<10}{}",
                " ",
                "test",
                format!("{lhs},"),
                rhs
            ),
            Self::SetG => writeln!(f, "{:>4}{:<10}al", " ", "setg"),
            Self::ProLog => {
                let push = format!("{:>4}{:<10}rbp", " ", "push");
                let mov = format!("{:>4}{:<10}{:<10}rsp", " ", "mov", "rbp,");
                writeln!(f, "{push}\n{mov}")
            }
            Self::Epilog => {
                let mov = format!("{:>4}{:<10}{:<10}rsp", " ", "mov", "rbp,");
                let pop = format!("{:>4}{:<10}rbp", " ", "pop");
                let ret = format!("{:>4}ret", " ");
                writeln!(f, "{mov}\n{pop}\n{ret}")
            }
            Self::Syscall => writeln!(f, "{:>4}syscall", " "),
        }
    }
}

impl Compile for ir::Instruction {
    fn compile(&self, state: &mut RegState, st: &SymbolTable) -> Vec<Instruction> {
        match self {
            ir::Instruction::LoadImm(i) => i.compile(state, st),
            ir::Instruction::CopyReg(i) => i.compile(state, st),
            ir::Instruction::DefFunc(i) => i.compile(state, st),
            ir::Instruction::Add(i) => i.compile(state, st),
            ir::Instruction::Sub(i) => i.compile(state, st),
            ir::Instruction::Mul(i) => i.compile(state, st),
            ir::Instruction::Div(i) => i.compile(state, st),
            ir::Instruction::Grt(i) => i.compile(state, st),
            ir::Instruction::Copy(i) => i.compile(state, st),
            ir::Instruction::Conditional(i) => i.compile(state, st),
            ir::Instruction::Jump(i) => i.compile(state, st),
            ir::Instruction::DefLabel(i) => i.compile(state, st),
            ir::Instruction::Call(i) => i.compile(state, st),
            ir::Instruction::Return(i) => i.compile(state, st),
            ir::Instruction::Enter(i) => i.compile(state, st),
            ir::Instruction::Leave(i) => i.compile(state, st),
        }
    }
}

// LoadImm(LoadImm),
impl Compile for ir::LoadImm {
    fn compile(&self, state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        let ir::LoadImm {
            des,
            imm: ir::Imm(imm),
        } = self;
        let reg = state.get_reg(des);
        vec![Instruction::MoveImm(reg, *imm)]
    }
}
// CopyReg(CopyReg),
impl Compile for ir::CopyReg {
    fn compile(&self, state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        let ir::CopyReg { des, src } = self;
        let des = state.get_reg(des);
        let src = state.get_reg(src);
        vec![Instruction::MoveReg(des, src)]
    }
}
// DefFunc(DefFunc),
impl Compile for ir::DefFunc {
    fn compile(&self, state: &mut RegState, st: &SymbolTable) -> Vec<Instruction> {
        let ir::DefFunc {
            name, params, body, ..
        } = self;
        let mut result = vec![Instruction::DefLabel(name.into())];
        let params = params
            .iter()
            .enumerate()
            .flat_map(|(idx, (reg, ty))| {
                let xreg = state.get_param_reg(reg);
                vec![
                    Instruction::MoveMemReg(
                        Mem::Param {
                            ty: *ty,
                            offset: (idx + 1) * 8,
                        },
                        xreg,
                    ),
                    Instruction::MoveRegMem(
                        xreg,
                        Mem::Param {
                            ty: *ty,
                            offset: (idx + 1) * 8,
                        },
                    ),
                ]
            })
            .collect::<Vec<Instruction>>();
        let mut body = body
            .iter()
            .flat_map(|inst| inst.compile(state, st))
            .collect::<Vec<Instruction>>();
        result.push(body.remove(0));
        result.extend_from_slice(&params);
        result.extend_from_slice(&body);
        // let ret_reg = state.get_ret_reg();
        // let last_reg = state.last_used_reg();
        // let instruction = Instruction::MoveReg(ret_reg, last_reg);
        // result.insert(result.len().saturating_sub(1), instruction);
        state.reset();
        result
    }
}
// Add(Add),
impl Compile for ir::Add {
    fn compile(&self, state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        let ir::Add { des, lhs, rhs } = self;
        let xdes = state.get_reg(des);
        let xlhs = state.get_reg(lhs);
        state.release_reg(lhs);
        let xrhs = state.get_reg(rhs);
        state.release_reg(rhs);
        vec![
            Instruction::Comment("Add".into()),
            Instruction::MoveReg(xdes, xlhs),
            Instruction::Add(xdes, xrhs),
        ]
    }
}
// Sub(Sub),
impl Compile for ir::Sub {
    fn compile(&self, state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        let ir::Sub { des, lhs, rhs } = self;
        let des = state.get_reg(des);
        let lhs = state.get_reg(lhs);
        let rhs = state.get_reg(rhs);
        vec![
            Instruction::Comment("Sub".into()),
            Instruction::MoveReg(des, lhs),
            Instruction::Sub(des, rhs),
        ]
    }
}
// Mul(Mul),
impl Compile for ir::Mul {
    fn compile(&self, state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        let ir::Mul { des, lhs, rhs } = self;
        let xdes = state.get_reg(des);
        let xlhs = state.get_reg(lhs);
        state.release_reg(lhs);
        let xrhs = state.get_reg(rhs);
        state.release_reg(rhs);
        vec![
            Instruction::Comment("Mul".into()),
            Instruction::MoveReg(xdes, xlhs),
            Instruction::Mul(xdes, xrhs),
        ]
    }
}
// Div(Div),
impl Compile for ir::Div {
    fn compile(&self, state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        let ir::Div { des, lhs, rhs } = self;
        let des = state.get_reg(des);
        let lhs = state.get_reg(lhs);
        let rhs = state.get_reg(rhs);
        vec![
            Instruction::Comment("Div".into()),
            Instruction::MoveReg(des, lhs),
            Instruction::Div(des, rhs),
        ]
    }
}

impl Compile for ir::Grt {
    fn compile(&self, state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        let ir::Grt { des, lhs, rhs } = self;
        let des = state.get_reg(des);
        let lhs = state.get_reg(lhs);
        let rhs = state.get_reg(rhs);
        vec![
            Instruction::Comment("Grt".into()),
            Instruction::MoveReg(des, lhs),
            Instruction::Cmp(des, rhs),
            Instruction::SetG,
            Instruction::MoveZx(des),
        ]
    }
}

impl Compile for ir::Copy {
    fn compile(&self, _state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        unimplemented!("{:?}", self)
    }
}
// Conditional(Conditional),
impl Compile for ir::Conditional {
    fn compile(&self, state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        let des = state.get_reg(&self.reg);
        vec![
            Instruction::Comment("Conditional".into()),
            Instruction::Test(des, des),
            Instruction::JumpZero(self.label.to_string()),
        ]
    }
}
// Jump(Jump),
impl Compile for ir::Jump {
    fn compile(&self, _state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        vec![
            Instruction::Comment("Jump".into()),
            Instruction::Jump(self.name()),
        ]
    }
}
// DefLabel(DefLabel),
impl Compile for ir::DefLabel {
    fn compile(&self, _state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        vec![
            Instruction::Comment("DefLabel".into()),
            Instruction::DefLabel(self.name()),
        ]
    }
}
// Call(Call),
impl Compile for ir::Call {
    fn compile(&self, _state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        vec![
            Instruction::Comment("Call".into()),
            Instruction::Call(self.caller.0.to_string()),
            // // FIXME: This is just a hot fix.... figure out a better way.
            // Instruction::MoveReg(X86Reg64::RDX.into(), X86Reg64::RAX.into()),
        ]
    }
}

// Return(Return),
impl Compile for ir::Return {
    fn compile(&self, state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        let Some(r) = self.0 else {
            return vec![];
        };
        let reg = state.get_reg(&r);
        let ret = state.get_ret_reg();
        vec![
            Instruction::Comment("Return".into()),
            Instruction::MoveReg(ret, reg),
        ]
    }
}

// Enter(Enter),
impl Compile for ir::Enter {
    fn compile(&self, _state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        vec![Instruction::Comment("Enter".into()), Instruction::ProLog]
    }
}
// Leave(Leave),
impl Compile for ir::Leave {
    fn compile(&self, _state: &mut RegState, _: &SymbolTable) -> Vec<Instruction> {
        vec![Instruction::Comment("Leave".into()), Instruction::Epilog]
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mem {
    Param { ty: ir::Type, offset: usize },
}
impl fmt::Display for Mem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Param { ty, offset } => write!(f, "{} [rbp-{offset}]", ty.size()),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::ir;
    // use crate::lexer::lex;
    // use crate::parse::parse;
    // use pretty_assertions::assert_eq;

    // fn setup(input: &str) -> Vec<Instruction> {
    //     lex(input)
    //         .and_then(parse)
    //         .and_then(ir::code_gen)
    //         .and_then(compile_ir_code)
    //         .unwrap()
    // }
    // #[test]
    // fn basic_test() {
    //     let left = setup("fn main() { 1 + 2; }");
    //     let right = vec![];
    //     assert_eq!(left, right);
    // }
}
