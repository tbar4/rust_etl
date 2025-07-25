/// Represents the Schema of a table or object
#[derive(Debug, Clone)]
pub struct Schema {
    pub columns: Vec<Column>
}

/// represents the column of a data table or structured dataset
#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: DataType
}

/// The Data Type of a column
#[derive(Debug, Clone)]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    Date,
    Json,
}