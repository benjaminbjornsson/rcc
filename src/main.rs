use clap::Parser;

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

fn main() {
    let args = Args::parse();

    println!("args: {args:?}");
}

