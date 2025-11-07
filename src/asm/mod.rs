pub mod codegen;
pub mod function;
pub mod instruction;
pub mod operand;
pub mod program;

pub enum Function {
    Function {
        name: String,
        instructions: InstrSeq,
    },
}

pub struct InstrSeq(pub Vec<Instr>);

pub enum Instr {
    Mov { src: Operand, dst: Operand },
    Ret,
}

pub enum Operand {
    Imm(i64),
    Register,
}

pub enum Program {
    Program(Function),
}
