use std::fmt::{write, Display};

use anyhow::Result;
use etl_core::extract::file::{csv::CSVFile, parse_file, FileInput, InputFile};
use clap::{Parser, ValueEnum};
use etl_utils::errors::ETLError;

#[derive(Debug, Parser)]
#[command(name="etl-cli", version, about="ETL Tool CLI")]
struct Args {
    /// Path to the source File
    #[arg(long, short = 'p')]
    source_path: String,
    
    /// The Source Type
    #[arg(long, short = 't')]
    source_type: SourceType,
}

#[derive(Debug, Clone, ValueEnum)]
enum SourceType {
    /// File Input
    FILE,
    Null
}

impl Default for SourceType {
    fn default() -> Self {
        SourceType::Null
    }
}

impl Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FILE => write!(f, "file"),
            Self::Null => write!(f, "No input type specified")
        }
    }
}

#[derive(Debug, Default)]
struct Config {
    source_path: String,
    source_type: String,
}


fn run_pipeline(config: Config) -> Result<()> {
    println!("Running ETL with config: {:?}", config);
    
    match config.source_type.to_lowercase().as_str() {
        "file" => {
            let input_file = InputFile::open(&config.source_path)?;
            let data = parse_file(input_file)?;
            match data {
                FileInput::CSV(file) => println!("{file}")
            }
        },
        "no input type specified" => eprintln!("No Input Type Specified"),
        _ => eprintln!("Not implemented yet")
    }
    
    Ok(())
}

fn main () -> Result<()> {
 let args = Args::parse();
 
 let config = Config {
     source_path: args.source_path,
     source_type: args.source_type.to_string(),
 };
 
 run_pipeline(config)?;
 
 Ok(())   
}