pub mod csv;
use csv::CSVFile;


use std::io::{BufReader, Read};
use etl_utils::errors::ETLError;

/// Define the different types of Files that are able to be parsed by the fileparser
#[derive(Debug)]
pub enum FileInput {
    CSV(CSVFile),
    //JSON(JSONFile),
    //TXT(TxtFile),
}

/// Create the trait used to parse the files
pub trait FileParser {
    fn parse(parser: BufReader<Box<dyn Read>>) -> Result<Self, ETLError> 
        where Self: Sized;
}

/// The standard struct for reading in files
pub struct InputFile {
    pub path: String,
    pub reader: BufReader<Box<dyn Read>>,
}

/// The implementation for opening and determining the file type
impl InputFile {
    /// Open the file and create the InputFile struct, passing the Reader
    pub fn open(path: &str) -> Result<Self, ETLError> {
        let file = std::fs::File::open(path)?;
        Ok(Self { 
            path: path.to_string(), 
            reader: BufReader::new(Box::new(file)) 
        })
    }
    
    /// Extract the file type to be used by our `parse_file` function
    pub fn file_type(&self) -> Option<&str> {
        std::path::Path::new(&self.path)
            .extension()
            .and_then(|t| t.to_str())
    }
}

pub fn parse_file(file: InputFile) -> Result<FileInput, ETLError> {
    match file.file_type() {
        Some("csv") => Ok(FileInput::CSV(CSVFile::parse(file.reader)?)),
        _ => Err(ETLError::UnsupportedFileType)
    }
}