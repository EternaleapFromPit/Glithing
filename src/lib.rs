mod ast;
mod borrowck;
mod bytecode;
mod cli;
mod compile;
mod cycles;
mod diagnostics;
mod leak;
mod lexer;
mod linker;
mod nuget;
mod llvm;
mod parser;
mod project;
pub mod realworld_smoke;
mod runtime;
mod tir;
mod toolchain;
mod workspace;
mod xunit_runner;

pub(crate) use cli::*;
pub(crate) use compile::*;
#[allow(unused_imports)]
pub(crate) use borrowck::BorrowChecker;
#[allow(unused_imports)]
pub(crate) use lexer::Lexer;
#[allow(unused_imports)]
pub(crate) use linker::link_package_sources;
#[allow(unused_imports)]
pub(crate) use nuget::{emit_nuget_package, NugetPackageSpec};
#[allow(unused_imports)]
pub(crate) use parser::Parser;
pub(crate) use project::*;
#[allow(unused_imports)]
pub(crate) use tir::TypedProgram;
pub(crate) use workspace::*;

pub fn run_cli() -> Result<(), String> {
    let cwd = std::env::current_dir()
        .map_err(|e| format!("failed to read current directory: {e}"))?;
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    run_cli_with_args_from(args, &cwd)
}

#[cfg(test)]
mod tests;
