pub struct AST {
    for_compiler: Vec<CompilerCommand>,
    program: Vec<Let>,
}

pub enum CompilerCommand {
    Include(String),
    Load(String),
}
pub struct Expr(Vec<Let>, Value);

pub struct Let(Name, Value, Type);

//$val: Ban Int * Nap String = 32 * '32'
//$val2: Int = Ban val
pub enum Value {
    Tuple(Vec<Value>),
    Either(Name, Box<Value>),
    Match(Box<Value>, Vec<(Name, Name, Value)>),
    Function(Vec<Name>, Box<Expr>),
    Application(Box<Value>, Vec<Value>),
    Constant(AtomicValue),
    Type(Box<Type>)
}

pub enum AtomicValue {
    Int(i32),
    StringLiteral(String),
}

pub enum Type {
    Product(Vec<(Name, Type)>),
    CoProduct(Vec<(Name, Box<Type>)>),
    Function(Box<Type>, Box<Type>),
    TypeVar(Name),
    Atomic(AtomicType),
}

pub enum AtomicType {
    Universe,
    Top,
    Bottom,
    Int,
    String,
}

pub struct Name {
    name: String,
    id: usize,
    context: Context,
}

pub enum Context {
    TypeContext,
    ValueContext,
    Constructor,
    None,
}