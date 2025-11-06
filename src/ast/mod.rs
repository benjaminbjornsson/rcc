pub mod exp;
pub mod function;
pub mod pretty;
pub mod program;
pub mod statement;

pub enum Program {
    FunctionDefinition(Function),
}

pub enum Function {
    Function { name: String, body: Statement },
}

pub enum Statement {
    Return(Exp),
}

pub enum Exp {
    ConstantInt(i64),
}
