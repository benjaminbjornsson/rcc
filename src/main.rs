use clap::Parser;
use std::io::Error;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

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

fn main() -> std::io::Result<()> {
    run(Args::parse())?;

    Ok(())
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

fn run(args: Args) -> Result<(), std::io::Error> {
    let input = &args.file_path;
    let pre = with_extension(input, "i");
    run_cmd("gcc", &["-E", "-P", input, "-o", &pre])?;

    let assembly = with_extension(input, "s");
    run_cmd(
        "gcc",
        &[
            "-S",
            "-O",
            "-fno-asynchronous-unwind-tables",
            "-fcf-protection=none",
            &pre,
            "-o",
            &assembly,
        ],
    )?;

    std::fs::remove_file(pre)?;

    if args.s {
        return Ok(())
    }

    let output = with_extension(input, "");
    run_cmd("gcc", &[&assembly, "-o", &output])?;

    std::fs::remove_file(assembly)?;

    Ok(())
}
