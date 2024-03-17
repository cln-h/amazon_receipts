use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use zip::read::ZipArchive;

#[derive(Debug)]
pub struct CsvLine {
    data: HashMap<String, String>,
}

impl CsvLine {
    pub fn new(headers: Vec<String>, values: Vec<String>) -> Self {
        let mut data = HashMap::new();

        for (header, value) in headers.into_iter().zip(values) {
            data.insert(header, value);
        }

        CsvLine { data }
    }
}

pub fn read_csv_from_reader<R: Read>(
    reader: R,
    pick_headers: &[String],
) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new().from_reader(reader);

    let mut lines: Vec<HashMap<String, String>> = Vec::new();
    for result in reader.records() {
        let record = result?;
        let mut data = HashMap::new();

        for (i, field) in record.iter().enumerate() {
            if let Some(header) = pick_headers.get(i) {
                data.insert(header.clone(), field.to_string());
            }
        }

        lines.push(data);
    }

    Ok(lines)
}

// TODO: Modify this to handle subdirectories
pub fn extract_file_from_zip(zip_file: &str, file_name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = std::fs::File::open(zip_file)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let filename = file.name().to_string();
        if filename == file_name {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;
            return Ok(contents);
        }
    }

    Err(format!("File '{}' not found in the zip archive", file_name).into())
}
