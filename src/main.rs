use clap::Parser;
use std::io::Error;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;
mod lexer;
mod token;
use lexer::Lexer;
pub mod error;
mod parser;
pub mod span;
use crate::error::CompilerError;
use crate::error::LexerError;
use error::AppError;
mod asm;
mod ast;
mod pretty;

/// Rust C Compiler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Run the lexer, but stop before parsing
    #[arg(long)]
    lex: bool,

    /// Run the lexer and parser, but stop before assembly generation
    #[arg(long)]
    parse: bool,

    /// Perform lexing, parsing, and assembly generation, but stop before code emission
    #[arg(long)]
    codegen: bool,

    /// Direct the compiler to emit an assembly file, but stops before assembler and linking.
    #[arg(short = 'S')]
    #[allow(non_snake_case)]
    s: bool,

    /// Print the AST with pretty-print.
    #[arg(long)]
    pretty_print: bool,

    /// Path to the file to compile.
    file_path: String,
}

fn main() -> Result<(), AppError> {
    run(Args::parse())
}

fn with_extension(path: &str, ext: &str) -> String {
    let mut buf = PathBuf::from(path);
    buf.set_extension(ext);
    buf.to_string_lossy().into_owned()
}

fn run_cmd(cmd: &str, args: &[&str]) -> Result<(), std::io::Error> {
    let out = Command::new(cmd)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("Failed to run {cmd}: {e}"));

    if out.status.success() {
        return Ok(());
    }

    let code = out.status.code().unwrap_or(-1);
    Err(Error::new(
        ErrorKind::Other,
        format!(
            "{cmd} exited with {code}\n{}",
            String::from_utf8_lossy(&out.stderr)
        ),
    ))
}

fn run_compiler(args: &Args, pre: &str) -> Result<Option<String>, CompilerError> {
    if args.lex {
        let lexer = Lexer::new(&pre);
        lexer.collect::<Result<Vec<_>, LexerError>>()?;

        return Ok(None);
    }

    if args.parse {
        let lexer = Lexer::new(&pre);
        let mut parser = parser::Parser::new(lexer);

        let ast = parser.parse()?;

        if args.pretty_print {
            println!("{}", ast);
        }

        return Ok(None);
    }

    let lexer = Lexer::new(&pre);
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse()?;
    let asm = asm::Program::from(ast);

    if args.codegen {
        if args.pretty_print {
            println!("{}", asm);
        }

        return Ok(None);
    }

    Ok(Some(asm.to_string()))
}

fn run(args: Args) -> Result<(), AppError> {
    let input = &args.file_path;
    let pre: String = with_extension(input, "i");
    run_cmd("gcc", &["-E", "-P", input, "-o", &pre])?;

    let pre_str: String = std::fs::read_to_string(&pre)?;
    std::fs::remove_file(pre)?;
    let assembly = with_extension(input, "s");
    match run_compiler(&args, &pre_str) {
        Ok(Some(res)) => std::fs::write(&assembly, &res)?,
        Err(CompilerError::Lexer(err)) => {
            error::render_diagnostic(&pre_str, &err);
            return Err(AppError::Compiler);
        }
        Err(CompilerError::Parser(err)) => {
            error::render_diagnostic(&pre_str, &err);
            return Err(AppError::Compiler);
        }
        _ => (),
    }

    if args.lex || args.parse || args.codegen {
        return Ok(());
    }

    if args.s {
        return Ok(());
    }

    let output = with_extension(input, "");
    run_cmd("gcc", &[&assembly, "-o", &output])?;

    std::fs::remove_file(&assembly)?;

    Ok(())
}
