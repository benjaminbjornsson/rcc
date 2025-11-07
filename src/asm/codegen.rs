use crate::asm::{Function, Instr, InstrSeq, Operand, Program};
use crate::ast;

impl From<ast::Program> for Program {
    fn from(prog: ast::Program) -> Self {
        match prog {
            ast::Program::FunctionDefinition(func) => Self::Program(Function::from(func)),
        }
    }
}

impl From<ast::Function> for Function {
    fn from(func: ast::Function) -> Self {
        match func {
            ast::Function::Function { name, body } => {
                let instructions = InstrSeq::from(body);

                Self::Function { name, instructions }
            }
        }
    }
}

impl From<ast::Statement> for InstrSeq {
    fn from(func: ast::Statement) -> Self {
        match func {
            ast::Statement::Return(exp) => {
                let mut instructions: Vec<Instr> = Vec::<Instr>::new();
                instructions.push(Instr::Mov {
                    src: Operand::Register,
                    dst: Operand::from(exp),
                });
                instructions.push(Instr::Ret);

                Self(instructions)
            }
        }
    }
}

impl From<ast::Exp> for Operand {
    fn from(exp: ast::Exp) -> Self {
        match exp {
            ast::Exp::ConstantInt(n) => Self::Imm(n),
        }
    }
}
