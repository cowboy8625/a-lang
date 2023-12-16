// use crate::lexer::Span;
use crate::parse::ast::{ExprBinary, ExprCall, ExprLit, ExprReturn, ExprVar, Ident};
use crate::parse::{Expr, ExprBlock, Item, ItemFn, Lit, Param, Statement};

use super::{Function, Scope, Symbol, SymbolTable, Type, Variable};

// pub type SemanticErrorList = Vec<SemanticError>;
//
// #[derive(Debug, Clone)]
// pub enum SemanticError {
//     UndefinedSymbol(String, Span),
// }

pub trait SemanticAnalysisVisitor {
    fn visit_lit(&mut self, _: &Lit) -> Type;
    fn visit_expr_binary(&mut self, _: &ExprBinary) -> Type;
    fn visit_expr_call(&mut self, call: &ExprCall) -> Type;
    fn visit_expr_var(&mut self, var: &ExprVar) -> Type;
    fn visit_expr_return(&mut self, var: &ExprReturn) -> Type;
    fn visit_expr(&mut self, _: &Expr) -> Type;
    fn visit_stmt(&mut self, _: &Statement);
    fn visit_block(&mut self, _: &ExprBlock);
    fn visit_params(&mut self, _: &[Param]);
    fn visit_item_fn(&mut self, _: &ItemFn);
    fn visit(&mut self, items: &[Item]) {
        for item in items {
            match item {
                // TODO: two pass for function delcaration
                Item::Fn(item_fn) => self.visit_item_fn(item_fn),
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SymbolTableBuilder {
    symbol_table: SymbolTable,
    scope: Vec<Scope>,
}

impl SymbolTableBuilder {
    pub fn build(self) -> SymbolTable {
        self.symbol_table
    }
}

impl SymbolTableBuilder {
    pub fn current_scope(&self) -> Scope {
        self.scope.last().copied().unwrap_or_default()
    }
}

impl SemanticAnalysisVisitor for SymbolTableBuilder {
    fn visit_lit(&mut self, lit: &Lit) -> Type {
        match lit {
            Lit::Int(_) => Type::U64,
            Lit::Bool(_) => todo!(),
            Lit::Str(_) => todo!(),
            Lit::Char(_) => todo!(),
        }
    }
    fn visit_expr_binary(&mut self, bin: &ExprBinary) -> Type {
        let left = self.visit_expr(&bin.left);
        let right = self.visit_expr(&bin.left);
        if left != right {
            todo!("log error")
        }
        left
    }
    fn visit_expr_call(&mut self, call: &ExprCall) -> Type {
        let Expr::Var(ExprVar {
            name: Ident { value, .. },
            ..
        }) = &*call.caller
        else {
            panic!("expected Ident found {:?}", call.caller);
        };
        let Some(Symbol::Function(function)) = self.symbol_table.get(value).cloned() else {
            todo!("log error undefined symbol")
        };

        if function.params.len() != call.args.len() {
            todo!("log error wrong number of arguments")
        }

        for (param, arg) in function.params.iter().zip(call.args.iter()) {
            let arg_type = self.visit_expr(arg);
            if param.ty != arg_type {
                todo!(
                    "log error type mismatch, expected {:?} found {:?}",
                    param.ty,
                    arg_type
                );
            }
        }

        function.ret
    }
    fn visit_expr_var(&mut self, var: &ExprVar) -> Type {
        let ExprVar { name, .. } = var;
        let Ident { value, .. } = name;
        let Some(Symbol::Variable(var)) = self.symbol_table.get(value) else {
            todo!("log error undefined symbol")
        };
        var.ty
    }
    fn visit_expr_return(&mut self, var: &ExprReturn) -> Type {
        self.visit_expr(&var.expr)
    }
    fn visit_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Lit(ExprLit { lit }) => self.visit_lit(lit),
            Expr::Binary(bin) => self.visit_expr_binary(bin),
            Expr::Call(call) => self.visit_expr_call(call),
            Expr::Var(var) => self.visit_expr_var(var),
            Expr::If(_if_expr) => todo!(),
            Expr::Block(block) => {
                // HACK: Need to return the last statements type
                self.visit_block(block);
                Type::Unit
            }
            Expr::Return(ret) => self.visit_expr_return(ret),
        }
    }
    fn visit_stmt(&mut self, stmt: &Statement) {
        self.visit_expr(&stmt.stmt);
    }
    fn visit_block(&mut self, block: &ExprBlock) {
        self.scope.push(Scope::Block);
        for stmt in &block.stmts {
            self.visit_stmt(stmt);
        }
        self.scope.pop();
    }

    fn visit_params(&mut self, params: &[Param]) {
        self.scope.push(Scope::Param);
        for param in params {
            let symbol = Variable {
                name: param.name.to_string(),
                scope: self.current_scope(),
                ty: Type::from(&param.kind),
                span: param.span,
            };
            self.symbol_table.insert(symbol.into());
        }
        self.scope.pop();
    }
    fn visit_item_fn(&mut self, item_fn: &ItemFn) {
        let symbol = Function {
            scope: Scope::Global,
            name: item_fn.name.to_string(),
            params: item_fn
                .params
                .iter()
                .map(|param| Variable {
                    name: param.name.to_string(),
                    scope: Scope::Param,
                    ty: Type::from(&param.kind),
                    span: param.span,
                })
                .collect(),
            ret: item_fn
                .ret_type
                .as_ref()
                .map(Type::from)
                .unwrap_or_default(),
            span: item_fn.span(),
        };

        self.symbol_table.insert(symbol.into());
        self.visit_params(&item_fn.params);
        self.visit_block(&item_fn.block);
    }
}
