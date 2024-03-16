use std::fs::File;
use std::io::{self, Read};
use zip::read::ZipArchive;

pub struct CsvFile {
  pub name: String,
  pub reader: csv::Reader<std::io::BufReader<std::fs::File>>,
}

impl CsvFile {
  
}
