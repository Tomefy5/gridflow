use gridflow_scheduler::{Scheduler, Cli, Commands};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let scheduler = Scheduler::new();
    
    match cli.command {
        Commands::Submit { task_type, input_file } => {
            println!("Submitting task: {} with input: {}", task_type, input_file);
            // TODO: impl soumission réelle
        }
        Commands::Status { task_id } => {
            println!("Checking status of task: {}", task_id);
            // TODO: impl statut réel
        }
    }
    
    // Optionnel: scheduler.run().await?;
    Ok(())
}
