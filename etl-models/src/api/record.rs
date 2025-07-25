/// Used to define a row or record like structure from a structured or semi-structured data source like a CSV, DB Row, or JSON file
#[derive(Debug, Clone)]
pub struct Record {
    pub fields: Vec<Field>
}

/// Represents a single field or cell within a `Record` and the associated Field Type
#[derive(Debug, Clone)]
pub enum Field {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null
}