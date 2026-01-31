use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    New {
        name: String,
        #[arg(short, long, default_value = "C")]
        language: String,
    },
    Run,
    Build,
    Clean,
}

#[derive(Parser)]
pub struct Arg {
    #[command(subcommand)]
    pub cmd: Commands,
}
