#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit,
    U64,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::U64 => write!(f, "u64"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
    Global,
    Param,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    Variable(Variable),
    Function(Function),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Variable>,
    pub ret: Type,
    pub span: Span,
}

#[derive(Debug)]
pub struct SymbolTable {
    pub variables: HashMap<String, Variable>,
    pub functions: HashMap<String, Function>,
}
