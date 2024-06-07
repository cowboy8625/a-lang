mod instruction;
#[cfg(test)]
mod test;
use std::collections::HashMap;

pub use instruction::*;

use crate::lexer::*;

use crate::parse::{
    Expr, ExprBinary, ExprBlock, ExprCall, ExprIf, ExprLet, ExprLit, ExprReturn, ExprVar, Ident,
    Item, ItemFn, Lit, LitBool, LitInt, Op, Param, Statement,
};
// use crate::semantic_analysis::{Symbol, SymbolTable, Variable};

use crate::symbol_table::SymbolTable;

pub fn code_gen(
    (ast, symbol_table): (Vec<Item>, SymbolTable),
) -> Result<(Vec<Instruction>, SymbolTable), Vec<String>> {
    let mut gen = IrGenerator::new(symbol_table);
    gen.visit(&ast);
    // for i in gen.code.iter() {
    //     match i {
    //         Instruction::DefFunc(DefFunc { name, params, body }) => {
    //             eprintln!("{name}: {params:?}, {body:?}")
    //         }
    //         Instruction::LoadImm(LoadImm { des, imm }) => eprintln!("{des:?}: {imm:?}"),
    //         Instruction::CopyReg(CopyReg { des, src }) => eprintln!("{des:?}: {src:?}"),
    //         Instruction::Add(Add { des, lhs, rhs }) => eprintln!("{des:?}: {lhs:?} + {rhs:?}"),
    //         Instruction::Sub(Sub { des, lhs, rhs }) => eprintln!("{des:?}: {lhs:?} - {rhs:?}"),
    //         Instruction::Mul(Mul { des, lhs, rhs }) => eprintln!("{des:?}: {lhs:?} * {rhs:?}"),
    //         Instruction::Div(Div { des, lhs, rhs }) => eprintln!("{des:?}: {lhs:?} / {rhs:?}"),
    //         Instruction::Grt(Grt { des, lhs, rhs }) => eprintln!("{des:?}: {lhs:?} > {rhs:?}"),
    //         Instruction::Copy(Copy { to, from }) => eprintln!("{to:?}: {from:?}"),
    //         Instruction::Conditional(Conditional { label, reg }) => eprintln!("{label:?}: {reg:?}"),
    //         Instruction::Jump(Jump(label)) => eprintln!("{label}"),
    //         Instruction::DefLabel(DefLabel(label)) => eprintln!("{label}:"),
    //         Instruction::Call(Call { caller, args, ret }) => {
    //             eprintln!("{caller}: {args:?}, {ret:?}")
    //         }
    //         Instruction::Return(Return(reg)) => eprintln!("return {reg:?}"),
    //         Instruction::Enter(_) => eprintln!("enter"),
    //         Instruction::Leave(_) => eprintln!("leave"),
    //     }
    // }
    Ok((gen.code, gen.symbol_table))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label(pub String);

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&Ident> for Label {
    fn from(value: &Ident) -> Self {
        Label(value.value())
    }
}

impl From<&str> for Label {
    fn from(value: &str) -> Self {
        Label(value.to_string())
    }
}

impl From<String> for Label {
    fn from(value: String) -> Self {
        Label(value)
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Reg(pub usize);

impl std::fmt::Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.0)
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Var(pub String);

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Imm(pub u64);

impl From<u64> for Imm {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Imm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

trait Ir {
    fn def_label(&mut self, label: Label);
    fn jump(&mut self, label: Label);
    fn load_imm(&mut self, imm: Imm) -> Reg;
    fn binary(&mut self, op: &Op, lhs: Reg, rhs: Reg) -> Reg;
    fn conditional(&mut self, label: Label, reg: Reg) -> Reg;
    fn call(&mut self, label: Label, args: Vec<Reg>, ret: Reg) -> Reg;
    fn early_return(&mut self, reg: Option<Reg>);
}

trait AstVisitor: Ir {
    fn visit_expr_var(&mut self, expr_var: &ExprVar) -> Reg;
    // FIXME: Not really what i wanted to do.
    fn visit_params(&mut self, expr: &Param) -> Reg;
    fn visit_expr_call(&mut self, expr_call: &ExprCall) -> Reg;
    fn visit_expr_binary(&mut self, bin: &ExprBinary) -> Reg;
    fn visit_item_fn(&mut self, item_fn: &ItemFn);
    fn visit_lit_int(&mut self, lit_int: &LitInt) -> Reg;
    fn visit_lit_bool(&mut self, lit_bool: &LitBool) -> Reg;
    fn visit_expr_if(&mut self, expr_if: &ExprIf) -> Reg;
    fn visit_expr_let(&mut self, expr_let: &ExprLet) -> Reg;

    fn visit_expr_return(&mut self, expr_ret: &ExprReturn) -> Reg {
        let ExprReturn { expr, .. } = expr_ret;
        let reg = self.visit_expr(expr);
        let r = if (**expr).is_call() { None } else { Some(reg) };
        self.early_return(r);
        self.jump(".exit".into());
        reg
    }

    fn visit_lit(&mut self, lit: &Lit) -> Reg {
        match lit {
            Lit::Int(ref lint) => self.visit_lit_int(lint),
            Lit::Bool(ref lbool) => self.visit_lit_bool(lbool),
            Lit::Str(_lstr) => unimplemented!(),
            Lit::Char(_lchar) => unimplemented!(),
        }
    }

    fn visit_expr_lit(&mut self, expr_lit: &ExprLit) -> Reg {
        let ExprLit { lit, .. } = &expr_lit;
        self.visit_lit(lit)
    }

    fn visit_expr(&mut self, expr: &Expr) -> Reg {
        match expr {
            Expr::Lit(ref elit) => self.visit_expr_lit(elit),
            Expr::Binary(ref ebinary) => self.visit_expr_binary(ebinary),
            Expr::Call(ref ecall) => self.visit_expr_call(ecall),
            Expr::Var(evar) => self.visit_expr_var(evar),
            Expr::If(eif) => self.visit_expr_if(eif),
            Expr::Block(eblock) => self.visit_expr_block(eblock),
            Expr::Return(ereturn) => self.visit_expr_return(ereturn),
            Expr::Let(_) => unimplemented!(),
        }
    }

    fn visit_stmt(&mut self, stmt: &Statement) -> Reg {
        let Statement { stmt, .. } = stmt;
        self.visit_expr(stmt)
    }

    fn visit_expr_block(&mut self, block: &ExprBlock) -> Reg {
        let mut reg: Option<Reg> = None;
        for stmt in block.stmts.iter() {
            reg = Some(self.visit_stmt(stmt));
        }
        let Some(reg) = reg else {
            panic!("WHAT DO I DO HERE!");
        };
        reg
    }

    fn visit(&mut self, items: &[Item]) {
        for item in items.iter() {
            match item {
                Item::Fn(ref item_fn) => self.visit_item_fn(item_fn),
            }
        }
    }
}

#[derive(Debug, Default)]
struct IrGenerator {
    code: Vec<Instruction>,
    block: Vec<Instruction>,
    reg_counter: usize,
    vars: HashMap<String, Reg>,
    return_call_regs: HashMap<String, Reg>,
    gen_label_number: usize,
    symbol_table: SymbolTable,
}

impl IrGenerator {
    fn new(symbol_table: SymbolTable) -> Self {
        Self {
            symbol_table,
            ..Default::default()
        }
    }
    fn push_to_block(&mut self, ir: impl Into<Instruction>) {
        self.block.push(ir.into());
    }

    fn push_fn(&mut self, ir: impl Into<Instruction>) {
        self.code.push(ir.into());
    }

    fn get_reg(&mut self) -> Reg {
        let reg = Reg(self.reg_counter);
        self.reg_counter += 1;
        reg
    }

    fn reset_regester_count(&mut self) {
        self.reg_counter = 0;
    }

    fn gen_label(&mut self) -> Label {
        let number = self.gen_label_number;
        self.gen_label_number += 1;
        Label(format!(".L{}", number))
    }
}

impl Ir for IrGenerator {
    fn def_label(&mut self, label: Label) {
        let instruction: Instruction = DefLabel(label).into();
        self.push_to_block(instruction);
    }
    fn jump(&mut self, label: Label) {
        let instruction: Instruction = Jump(label).into();
        self.push_to_block(instruction);
    }

    fn load_imm(&mut self, imm: Imm) -> Reg {
        let des = self.get_reg();
        let load = LoadImm { des, imm };
        self.push_to_block(load);
        des
    }

    fn binary(&mut self, op: &Op, lhs: Reg, rhs: Reg) -> Reg {
        let des = self.get_reg();
        let instruction: Instruction = match op {
            Op::Add(_) => Add { des, lhs, rhs }.into(),
            Op::Sub(_) => Sub { des, lhs, rhs }.into(),
            Op::Mul(_) => Mul { des, lhs, rhs }.into(),
            Op::Div(_) => Div { des, lhs, rhs }.into(),
            Op::Grt(_) => Grt { des, lhs, rhs }.into(),
            _ => unimplemented!("{op:?}"),
        };
        self.push_to_block(instruction);
        des
    }

    fn conditional(&mut self, label: Label, reg: Reg) -> Reg {
        let instruction: Instruction = Conditional { label, reg }.into();
        self.push_to_block(instruction);
        reg
    }

    fn call(&mut self, caller: Label, args: Vec<Reg>, ret: Reg) -> Reg {
        let instruction: Instruction = Call { caller, args, ret }.into();
        self.push_to_block(instruction);
        ret
    }

    fn early_return(&mut self, reg: Option<Reg>) {
        let instruction: Instruction = Return(reg).into();
        self.push_to_block(instruction);
    }
}

impl AstVisitor for IrGenerator {
    fn visit_expr_var(&mut self, expr_var: &ExprVar) -> Reg {
        let ExprVar { name, .. } = expr_var;
        *self.vars.get(&name.value()).unwrap()
    }

    fn visit_params(&mut self, params: &Param) -> Reg {
        let Param { name, .. } = params;
        let des = self.get_reg();
        self.vars.insert(name.value(), des);
        des
    }

    fn visit_expr_call(&mut self, expr_call: &ExprCall) -> Reg {
        let ExprCall { caller, args, .. } = expr_call;
        let Expr::Var(ExprVar { name, .. }) = &**caller else {
            panic!("expected Ident");
        };
        // FIXME: this reg needs to be stored with var in discriper?
        let ret = self.get_reg();
        self.return_call_regs.insert(name.to_string(), ret);
        let args = args
            .iter()
            .map(|expr| self.visit_expr(expr))
            .collect::<Vec<Reg>>();
        self.call(name.into(), args, ret)
    }

    fn visit_expr_binary(&mut self, bin: &ExprBinary) -> Reg {
        let ExprBinary {
            left, right, op, ..
        } = bin;
        let lhs = self.visit_expr(left);
        let rhs = self.visit_expr(right);
        self.binary(op, lhs, rhs)
    }

    fn visit_item_fn(&mut self, item_fn: &ItemFn) {
        let ItemFn {
            name,
            params,
            block,
            ret_type: _,
            ..
        } = item_fn;

        self.gen_label_number = 0;
        self.reset_regester_count();
        let params = params
            .iter()
            .map(|p| (self.visit_params(p), Type::U64))
            .collect();

        self.push_to_block(Enter);
        let _reg = self.visit_expr_block(block);
        self.def_label(".exit".into());
        self.push_to_block(Leave);

        let body = self.block.clone();
        self.block.clear();
        self.push_fn(DefFunc {
            name: name.value(),
            params,
            body,
        });
    }

    fn visit_lit_int(&mut self, lit_int: &LitInt) -> Reg {
        let imm: Imm = lit_int.parse::<u64>().unwrap().into();
        self.load_imm(imm)
    }

    fn visit_lit_bool(&mut self, lit_bool: &LitBool) -> Reg {
        let num: bool = lit_bool.parse::<bool>().unwrap();
        let imm: Imm = (num as u64).into();
        self.load_imm(imm)
    }

    fn visit_expr_let(&mut self, expr_let: &ExprLet) -> Reg {
        let ExprLet { expr, .. } = expr_let;
        // let Some(Symbol::Variable(var)) = self._symbol_table.get(&name.value()) else {
        //     panic!("expected symbol");
        // };
        // FIXME: ssa variables are just regesters.
        // I guess we need to store the variable name
        // with the register its origanly bound to?
        //
        // let Variable {
        //     scope,
        //     name,
        //     ty,
        //     span,
        // } = var;
        self.visit_expr(expr)
    }

    fn visit_expr_if(&mut self, expr_if: &ExprIf) -> Reg {
        let ExprIf {
            if_token: _,
            cond,
            then_branch,
            else_branch,
        } = expr_if;
        let cond_reg = self.visit_expr(cond);
        let label = self.gen_label();
        let des = self.conditional(label.clone(), cond_reg);
        self.visit_expr_block(then_branch);
        self.def_label(label);
        if let Some((_, else_branch)) = else_branch {
            self.visit_expr(else_branch);
        }
        des
    }
}
