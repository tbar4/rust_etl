use super::FileParser;
use std::io::{BufReader, Read};
use etl_utils::errors::ETLError;
use std::fmt::Display;

const COLUMN_WIDTH: usize = 14;

#[derive(Debug)]
pub struct CSVFile{
    pub rows: Vec<Vec<String>>,
}

impl FileParser for CSVFile {
    fn parse(mut reader: BufReader<Box<dyn Read>>) -> Result<Self, ETLError> 
            where Self: Sized {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        
        let csv = buf
            .lines()
            .map(|line| line.split(",").map(|cell| cell.trim().to_string()).collect())
            .collect();
        
        Ok( Self { rows: csv })
    }
}

impl Display for CSVFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = String::new();
        display.push('|');
        
        for row in self.rows.iter() {
            for cell in row.iter() {
                let cell = truncate_if_greater_than_col_width(cell);
                let content = if cell.chars().all(|c| c.is_numeric()) {
                    format!("{:>width$}|", cell, width = COLUMN_WIDTH)
                } else {
                    format!("{:<width$}|", cell, width = COLUMN_WIDTH)
                };
                display.push_str(&content);
            }
            display.push_str("\n|");
        }
        
        write!(f, "{display}")
    }
}


fn truncate_if_greater_than_col_width(cell: &String) -> String {
    if cell.len() > COLUMN_WIDTH {
        let trimmed_cell: String = cell.chars().take(COLUMN_WIDTH - 4).collect();
        let cell = format!("{trimmed_cell}...");
        cell
    } else {
        cell.to_string()
    }
}