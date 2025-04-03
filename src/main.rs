use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    Monitor,
}

impl Commands {
    pub fn execute(self) {
        match self {
            Self::Monitor => {
                println!("[TODO] Power: ----W")
            }
        }
    }
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn execute(self) {
        self.command.execute();
    }
}

fn main() {
    let cli = Cli::parse();
    cli.execute();
}
