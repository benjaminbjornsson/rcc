use clap::Parser;
use std::io::Error;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;
mod lexer;
use lexer::Lexer;

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

    /// Path to the file to compile.
    file_path: String,
}

fn main() -> Result<(), CompilerError> {
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

#[derive(Debug)]
enum CompilerError {
    Io(std::io::Error),
    Lexer,
}

impl From<std::io::Error> for CompilerError {
    fn from(e: std::io::Error) -> Self {
        CompilerError::Io(e)
    }
}

impl From<lexer::error::LexerError> for CompilerError {
    fn from(_: lexer::error::LexerError) -> Self {
        CompilerError::Lexer
    }
}

fn run_compiler(args: &Args, pre: &str, assembly: &str) -> Result<(), CompilerError> {
    let pre_str: String = std::fs::read_to_string(&pre)?;
    std::fs::remove_file(pre)?;

    let lexer = Lexer::new(&pre_str);
    if args.lex {
        for token in lexer {
            token?;
        }
        return Ok(());
    }

    Ok(())
}

fn run(args: Args) -> Result<(), CompilerError> {
    let input = &args.file_path;
    let pre = with_extension(input, "i");
    run_cmd("gcc", &["-E", "-P", input, "-o", &pre])?;

    let assembly = with_extension(input, "s");
    run_compiler(&args, &pre, &assembly)?;

    if args.lex || args.parse || args.codegen {
        return Ok(());
    }

    if args.s {
        return Ok(());
    }

    let output = with_extension(input, "");
    run_cmd("gcc", &[&assembly, "-o", &output])?;

    std::fs::remove_file(assembly)?;

    Ok(())
}
