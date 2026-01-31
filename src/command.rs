use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Create a new project")]
    New {
        name: String,
        #[arg(short, long, default_value = "C")]
        language: String,
    },
    #[command(about = "Run the project")]
    Run,
    #[command(about = "Build the project")]
    Build,
    #[command(about = "Clean the project")]
    Clean,
}

#[derive(Parser)]
pub struct Arg {
    #[command(subcommand)]
    pub cmd: Commands,
}
