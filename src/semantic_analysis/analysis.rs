use crate::lexer::Span;
use crate::parse::ast::ExprLit;
use crate::parse::{
    Expr, ExprBlock, Item, ItemFn, Lit, LitBool, LitChar, LitInt, LitStr, Param, Statement,
};

use super::{Function, Scope, Symbol, SymbolTable, Type, Variable};

#[derive(Debug, Clone)]
pub struct SemanticErrorList {
    errors: Vec<SemanticError>,
}

#[derive(Debug, Clone)]
pub enum SemanticError {
    UndefinedSymbol(String, Span),
}

trait SemanticAnalysisVisitor {
    fn visit_lit(&mut self, _: &Lit) -> Type;
    fn visit_expr(&mut self, _: &Expr);
    fn visit_stmt(&mut self, _: &Statement);
    fn visit_block(&mut self, _: &ExprBlock);
    fn visit_params(&mut self, _: &[Param]);
    fn visit_item_fn(&mut self, _: &ItemFn);
    fn visit(&mut self, items: &[Item]) {
        for item in items {
            match item {
                Item::Fn(item_fn) => self.visit_item_fn(item_fn),
            }
        }
    }
}

pub struct SymbolTableBuilder {
    symbol_table: SymbolTable,
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
    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Lit(ExprLit { lit }) => {
                // TODO: Not sure if we need this at all
                let _ = self.visit_lit(lit);
            }
            Expr::Binary(bin) => todo!(),
            Expr::Call(call) => todo!(),
            Expr::Var(var) => todo!(),
            Expr::If(if_expr) => todo!(),
            Expr::Block(block) => todo!(),
            Expr::Return(ret) => todo!(),
        }
    }
    fn visit_stmt(&mut self, stmt: &Statement) {
        self.visit_expr(&stmt.stmt);
    }
    fn visit_block(&mut self, block: &ExprBlock) {
        for stmt in &block.stmts {
            self.visit_stmt(stmt);
        }
    }

    fn visit_params(&mut self, params: &[Param]) {
        for param in params {
            let symbol = Variable {
                name: param.name.to_string(),
                scope: Scope::Param,
                ty: Type::from(&param.kind),
                span: param.span,
            };
            self.symbol_table.insert(symbol.into());
        }
    }
    fn visit_item_fn(&mut self, item_fn: &ItemFn) {
        let symbol = Function {
            scope: Scope::Global,
            name: item_fn.name.to_string(),
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
