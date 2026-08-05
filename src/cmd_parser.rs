use clap::{Parser, Subcommand};

use crate::{errors::JobError, string_lexer::App};

#[derive(Parser)]
#[command(name = "pp")]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  List,
  Show,
  Kill,
  Stop,
  Run {
    // Get each single quote string as a String into the args vector.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
  },
}

impl App {
  pub fn argv_parser(cli: &Cli) -> std::result::Result<(), JobError> {
    loop {
      match &cli.command {
        Commands::Run { args } => {
          // Get the string item.
          for item in args.iter() {
            // lex it
            let argv = App::string_lexer(item.as_str());

            if argv.is_empty() {
              continue;
            }

            // summon a process
            std::process::Command::new(&argv[0])
              .args(&argv[1..])
              .status().map_err(|e| {
                JobError::SpawnFailed { cmd: argv[0].clone(), source: e }
              })?;
          }
        }

        Commands::Kill => return Ok(()),
        _ => {
          println!("None")
        }
      }
    }
  }
}
