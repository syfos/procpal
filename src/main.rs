use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pp")]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

fn string_lexer(str: &str) -> Vec<String> {
  shlex::split(str).unwrap_or_default()
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();
  match &cli.command {
    Commands::Run { args } => {
      // Get the string item.
      for item in args.iter() {
        // lex it
        let argv = string_lexer(item.as_str());

        if argv.is_empty() {
          continue;
        }

        // summon a process
        std::process::Command::new(&argv[0])
          .args(&argv[1..])
          .status()?;
      }
    }
    _ => {
      println!("None")
    }
  }
  Ok(())
}
