use clap::{Parser, Subcommand};
use dotenvy::dotenv;

#[derive(Parser)]
#[command(name = "takdir-admin")]
#[command(about = "Takdir Administration CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Seed the initial system state (e.g. creating the admin user)
    Seed,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Seed => {
            println!("Database seeding command called.");
            // Database seeding implementation will be added in future tasks
        }
    }

    Ok(())
}
