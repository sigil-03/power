use clap::{Parser, Subcommand};

mod control;
mod monitor;
mod system;
mod tasmota;
mod types;

#[derive(Subcommand)]
pub enum Commands {
    Monitor,
    #[command(subcommand)]
    Set(types::PowerState),
}

impl Commands {
    pub async fn execute(self, config_file: &str) {
        let handle = match self {
            Self::Monitor => {
                let s = system::System::new_from_file(config_file).unwrap();
                tokio::spawn(async move {
                    s.get_power().await.unwrap();
                })
            }
            Self::Set(state) => {
                // let c = Controller::new_from_file(config_file).unwrap();
                tokio::spawn(async move {
                    println!("SET");
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
