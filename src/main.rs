use crate::{cmd_parser::Cli, string_lexer::App};
use clap::Parser;

pub mod cmd_parser;
pub mod string_lexer;
pub mod errors;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();
  App::argv_parser(&cli)?;
  Ok(())
}
