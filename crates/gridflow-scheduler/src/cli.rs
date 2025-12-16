use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gridflow")]
#[command(about = "GridFlow Distributed Scheduler CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Submit {
        #[arg(short, long)]
        task_type: String,
        #[arg(short, long)]
        input_file: String,
    },
    Status {
        #[arg(short, long)]
        task_id: String,
    },
}
