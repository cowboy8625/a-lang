use super::{
    keyword, Ctrl, CtrlColon, CtrlComma, CtrlLBrace, CtrlLParan, CtrlRBrace, CtrlRParan,
    CtrlRightArrow, CtrlSemiColon, Expr, ExprBinary, ExprBlock, ExprCall, ExprIf, ExprLet,
    ExprReturn, Ident, Item, ItemFn, LitBool, LitChar, LitInt, LitStr, Op, OpAdd, OpDiv, OpEqual,
    OpEqualEqual, OpGeq, OpGrt, OpLeq, OpLes, OpMul, OpNeq, OpSub, Param, Statement, Type,
};
use crate::symbol_table::{Scope, Symbol, SymbolData, SymbolTable, SymbolType, TypeName};

use crate::lexer::{Span, Token, TokenStream};
type PResult<T> = std::result::Result<T, String>;

pub struct Parser {
    stream: TokenStream,
    errors: Vec<String>,
    symbol_table: SymbolTable,
    scope: Vec<Scope>,
}

// declaration
// statement
// expression
// equality
// comparison
// term
// factor
// unary
// primary

impl Parser {
    pub fn new(stream: TokenStream) -> Self {
        Self {
            stream,
            errors: vec![],
            symbol_table: SymbolTable::new(),
            scope: vec![Scope::default()],
        }
    }

    pub fn parse(mut self) -> Result<(Vec<Item>, SymbolTable), Vec<String>> {
        let mut ast = vec![];
        while self.stream.is_not_at_end() {
            match self.program() {
                Ok(item) => {
                    ast.push(item);
                }
                Err(error) => {
                    self.errors.push(error);
                    self.recover();
                }
            }
        }
        if !self.errors.is_empty() {
            return Err(self.errors);
        }
        Ok((ast, self.symbol_table.clone()))
    }

    fn current_scope(&self) -> Scope {
        self.scope.last().cloned().unwrap_or_default()
    }

    fn insert_symbol(
        &mut self,
        name: impl Into<String>,
        ty: SymbolType,
        type_name: TypeName,
        span: Span,
    ) {
        self.symbol_table.insert(
            Symbol {
                name: name.into(),
                scope: self.current_scope(),
            },
            SymbolData {
                ty,
                scope: self.current_scope(),
                type_name,
                span,
            },
        );
    }

    fn recover(&mut self) {
        let Some(error) = self.errors.last() else {
            return;
        };
        match error.as_str() {
            "expected fn" => unimplemented!("expected fn"),
            "expected a ident" => unimplemented!("expected a ident"),
            "expected return type" => unimplemented!("expected a ident"),
            "expected '('" => unimplemented!("expected a ident"),
            "expected ':' after function param id" => unimplemented!("expected a ident"),
            "expected '{'" => unimplemented!("expected a ident"),
            "expected '}'" => unimplemented!("expected a ident"),
            "functions params end with ')'" => unimplemented!("expected a ident"),
            _ => unimplemented!("{error}"),
        }
    }

    fn expr_next_if<Expected>(&mut self) -> Option<Expr>
    where
        Expected: Token + Clone,
        Expr: From<Expected>,
    {
        self.stream
            .next_if::<Expected>()
            .cloned()
            .map(|i| Expr::from(i))
    }

    pub fn op_next_if<Expected>(&mut self) -> Option<Op>
    where
        Expected: Token + Clone,
        Op: From<Expected>,
    {
        self.stream
            .next_if::<Expected>()
            .map(|i| Op::from((*i).clone()))
    }

    pub fn ctrl_next_if<Expected>(&mut self) -> Option<Ctrl>
    where
        Expected: Token + Clone,
        Ctrl: From<Expected>,
    {
        self.stream
            .next_if::<Expected>()
            .map(|i| Ctrl::from((*i).clone()))
    }

    fn program(&mut self) -> PResult<Item> {
        self.declaration()
    }

    fn declaration(&mut self) -> PResult<Item> {
        self.item_fn()
    }

    fn item_fn(&mut self) -> PResult<Item> {
        let keyword_fn = self
            .stream
            .next_if::<keyword::Fn>()
            .cloned()
            .ok_or::<String>("expected fn".into())?;
        let name = self
            .stream
            .next_if::<Ident>()
            .ok_or::<String>("expected a ident".into())?
            .clone();

        let func_name = name.value();
        let func_scope = self.current_scope();
        let func_ty = SymbolType::Function;
        let func_span = name.span();

        self.scope.push(Scope::Function(name.value()));

        let params = self.params()?;
        let ret_type = self.ret_type()?;
        let block = self.block()?;

        self.symbol_table.insert(
            Symbol {
                name: func_name,
                scope: func_scope.clone(),
            },
            SymbolData {
                ty: func_ty,
                scope: func_scope.clone(),
                type_name: ret_type.clone().map(|t| t.into()).unwrap_or(TypeName::Null),
                span: func_span,
            },
        );

        self.scope.pop();

        Ok(Item::Fn(ItemFn::new(
            keyword_fn, name, params, block, ret_type,
        )))
    }

    fn ret_type(&mut self) -> PResult<Option<Type>> {
        let Some(_) = self.stream.next_if::<CtrlRightArrow>() else {
            return Ok(None);
        };
        let Some(t) = self.stream.next_if::<Ident>() else {
            return Err("expected return type".into());
        };

        Ok(Some(t.into()))
    }

    fn params(&mut self) -> PResult<Vec<Param>> {
        self.stream
            .next_if::<CtrlLParan>()
            .ok_or::<String>("expected '('".into())?;
        let mut params = vec![];
        while !self.stream.is_peek_a::<CtrlRParan>() {
            let Some(name) = self.stream.next_if::<Ident>().cloned() else {
                break;
            };

            self.ctrl_next_if::<CtrlColon>()
                .ok_or::<String>("expected ':' after function param id".into())?;

            let Some(kind) = self.stream.next_if::<Ident>().cloned() else {
                break;
            };

            self.insert_symbol(
                name.value(),
                SymbolType::Parameter,
                kind.clone().into(),
                name.span(),
            );

            params.push((&name, &kind).into());
            // grabs trailing commas.
            self.stream.next_if::<CtrlComma>();
        }

        if self.stream.next_if::<CtrlRParan>().is_none() {
            return Err("functions params end with ')'".into());
        }
        Ok(params)
    }

    fn block(&mut self) -> PResult<ExprBlock> {
        let left_brace = self
            .stream
            .next_if::<CtrlLBrace>()
            .cloned()
            .ok_or::<String>("expected '{'".into())?;
        let mut stmts = vec![];
        while !self.stream.is_peek_a::<CtrlRBrace>() {
            let stmt = self.statement()?;
            stmts.push(stmt);
        }
        let right_brace = self
            .stream
            .next_if::<CtrlRBrace>()
            .cloned()
            .ok_or::<String>("expected '}'".into())?;
        Ok(ExprBlock::new(left_brace, right_brace, stmts))
    }

    fn statement(&mut self) -> PResult<Statement> {
        let stmt = self.let_expression()?;
        let span = stmt.span();
        Ok(Statement { stmt, span })
    }

    fn let_expression(&mut self) -> PResult<Expr> {
        let Some(let_token) = self.stream.next_if::<keyword::Let>().cloned() else {
            return self.expr_return();
        };
        let name = self
            .stream
            .next_if::<Ident>()
            .ok_or::<String>("expected a ident".into())?
            .clone();
        let eq_token = self
            .stream
            .next_if::<OpEqual>()
            .cloned()
            .ok_or::<String>("expected '='".into())?;
        // TODO: probably guess the type of the expression
        let expr = self.expression();

        self.insert_symbol(
            name.value(),
            SymbolType::Variable,
            // TODO: Get type from expr
            TypeName::U64,
            name.span(),
        );

        Ok(ExprLet {
            let_token,
            name,
            eq_token: eq_token.into(),
            expr: expr.into(),
        }
        .into())
    }

    fn expr_return(&mut self) -> PResult<Expr> {
        let ret = self.stream.next_if::<keyword::Return>().copied();
        let Some(ret) = ret else {
            return Ok(self.expression());
        };
        let expr = self.expression();
        self.stream
            .next_if::<CtrlSemiColon>()
            .ok_or::<String>("return statements end in ';'".into())?;
        Ok(ExprReturn::new(ret, expr).into())
    }

    // NOTE: Probably best that these functions return a Option over a Result cause then functions
    // will do there own error reporting at the point of the error.
    // Something like
    // ```
    // self.report(Error::MissingSimiColon(span))
    // ```
    fn if_expression(&mut self) -> Expr {
        if self.stream.peek::<keyword::If>().is_some() {
            // HACK: this implemention is a bit of a hack with all the funcitons not returning a
            // Result.
            let if_token = self.stream.next_as::<keyword::If>().cloned().unwrap();
            let cond = Box::new(self.comparison());
            let then_branch = self.block().expect("failed to get block");
            let else_branch = self.else_branch();
            return ExprIf::new(if_token, cond, then_branch, else_branch).into();
        }
        self.comparison()
    }

    fn else_branch(&mut self) -> Option<(keyword::Else, Box<Expr>)> {
        let keyword_else = self.stream.next_if::<keyword::Else>().cloned()?;
        let block = if self.stream.peek::<keyword::If>().is_some() {
            self.if_expression()
        } else {
            // HACK: Fix this expected
            Expr::Block(self.block().expect("failed to get block"))
        };
        Some((keyword_else, Box::new(block)))
    }

    fn expression(&mut self) -> Expr {
        self.if_expression()
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while let Some(op) = self
            .op_next_if::<OpGrt>()
            .or(self.op_next_if::<OpLes>())
            .or(self.op_next_if::<OpGeq>())
            .or(self.op_next_if::<OpLeq>())
            .or(self.op_next_if::<OpEqualEqual>())
            .or(self.op_next_if::<OpNeq>())
        {
            let right = self.term();
            expr = Expr::from(ExprBinary::from((expr, right, op)))
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while let Some(op) = self.op_next_if::<OpSub>().or(self.op_next_if::<OpAdd>()) {
            let right = self.factor();
            expr = Expr::from(ExprBinary::from((expr, right, op)))
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.call();
        while let Some(op) = self.op_next_if::<OpMul>().or(self.op_next_if::<OpDiv>()) {
            let right = self.call();
            expr = Expr::from(ExprBinary::from((expr, right, op)))
        }
        expr
    }

    fn call(&mut self) -> Expr {
        let mut expr = self.primary();

        if let Some(left_paran) = self.stream.next_if::<CtrlLParan>().cloned() {
            expr = self.finish_call(expr, left_paran);
        }

        expr
    }

    fn finish_call(&mut self, caller: Expr, left_paran: CtrlLParan) -> Expr {
        let mut args = vec![];
        if !self.stream.is_peek_a::<CtrlRParan>() {
            while !self.stream.is_peek_a::<CtrlRParan>() {
                args.push(self.expression());
                if self.stream.next_if::<CtrlComma>().is_none() {
                    break;
                };
            }
        }
        let Some(right_paran) = self.stream.next_if::<CtrlRParan>().cloned() else {
            // TODO: make this report an error
            panic!("expected a right paran");
        };
        Expr::Call(ExprCall::new(
            Box::new(caller),
            left_paran,
            args,
            right_paran,
        ))
    }

    fn primary(&mut self) -> Expr {
        let Some(expr) = self
            .expr_next_if::<LitInt>()
            .or(self.expr_next_if::<LitBool>())
            .or(self.expr_next_if::<LitStr>())
            .or(self.expr_next_if::<LitChar>())
            .or(self.expr_next_if::<Ident>())
        else {
            // TODO: make this report an error
            panic!("unknown expression '{:?}'", self.stream.peek_blind());
        };
        expr
    }
}
