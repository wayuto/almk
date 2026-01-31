mod build;
mod command;
mod config;
mod new;
use clap::Parser;
use command::{Arg, Commands};

use crate::{build::build, new::new};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arg::parse();

    match args.cmd {
        Commands::New { name, language } => match language.as_str() {
            "C" => {
                new(
                    name.clone(),
                    language.clone(),
                    Some("clang".to_owned()),
                    None,
                    "clang".to_owned(),
                )?;
                println!("'{}' has been created successfully.", name);
            }
            "C++" => {
                new(
                    name.clone(),
                    language.clone(),
                    None,
                    Some("clang++".to_owned()),
                    "clang++".to_owned(),
                )?;
                println!("'{}' has been created successfully.", name);
            }
            _ => {
                return Err(format!("Unsupported language: {:?}", language).into());
            }
        },
        Commands::Build => build(true)?,
        Commands::Clean => {
            if std::fs::metadata(&"target/objects").is_ok() {
                std::fs::remove_dir_all(&"target/objects")?;
                println!("Removed object files in 'target/objects' directory.");
            } else {
                println!("No 'target/objects' directory found to clean.");
            }
        }
        Commands::Run => {
            build(false)?;
            let toml_string = std::fs::read_to_string(&"./Alumake.toml")?;
            let config: config::Config = toml::from_str(&toml_string)?;
            let name = config.package.name;
            let status = std::process::Command::new(format!("./target/{}", name)).status()?;
            if !status.success() {
                return Err(format!("Failed to run the executable: {:?}", name).into());
            }
        }
    }
    Ok(())
}
