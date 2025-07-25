use thiserror::Error;

#[derive(Error, Debug)]
pub enum ETLError {
    #[error("Failed to read the file")]
    ReadError(#[from] std::io::Error),
    #[error("No input type provided, please specify --type <TYPE>")]
    NoFileTypeProvided,
    #[error("Unsupported File Type")]
    UnsupportedFileType,
    #[error("DataType mismatch")]
    FieldTypeError
}