use clap::{Parser, Subcommand};
use monitor::Monitor;

mod monitor;

#[derive(Subcommand)]
pub enum Commands {
    Monitor,
}

impl Commands {
    pub fn execute(self, config_file: &str) {
        match self {
            Self::Monitor => {
                let _m = Monitor::new_from_file(config_file).unwrap();

                println!("[TODO] Power: ----W")
            }
        }
    }
}

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    config_file: String,
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn execute(self) {
        self.command.execute(&self.config_file);
    }
}

fn main() {
    let cli = Cli::parse();
    cli.execute();
}
