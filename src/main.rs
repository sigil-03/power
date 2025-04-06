use clap::{Parser, Subcommand};

mod control;
mod monitor;
mod system;
mod tasmota;
mod types;

#[derive(Parser)]
pub struct PowerCommand {
    index: usize,
    #[command(subcommand)]
    state: types::PowerState,
}

#[derive(Subcommand)]
pub enum Commands {
    Monitor,
    Set(PowerCommand),
}

impl Commands {
    pub async fn execute(self, config_file: &str) {
        let s = system::System::new_from_file(config_file).unwrap();

        let handle = match self {
            Self::Monitor => tokio::spawn(async move {
                s.try_get_power().await.unwrap();
            }),
            Self::Set(command) => {
                // let c = Controller::new_from_file(config_file).unwrap();
                tokio::spawn(async move {
                    s.try_set_power(command.index, command.state).await.unwrap();
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
