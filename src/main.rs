use clap::{Parser, Subcommand};
use monitor::Monitor;

mod monitor;
mod tasmota;

#[derive(Subcommand)]
pub enum Commands {
    Monitor,
}

impl Commands {
    pub async fn execute(self, config_file: &str) {
        let handle = match self {
            Self::Monitor => {
                let m = Monitor::new_from_file(config_file).unwrap();
                tokio::spawn(async move {
                    m.get_power().await.unwrap();
                })
            }
        };
        handle.await.unwrap();
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
    pub async fn execute(self) {
        self.command.execute(&self.config_file).await;
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    cli.execute().await;
}
