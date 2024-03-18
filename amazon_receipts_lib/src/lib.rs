use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

fn read_csv_from_reader<R: Read>(
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
fn extract_file_from_zip(zip_filepath: &str, file_name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = std::fs::File::open(zip_filepath)?;
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

pub fn build_receipts(zip_filepath: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
  let pick_headers = vec! [
    "Order ID".to_string(),
    "Order Date".to_string(),
    "Currency".to_string(),
    "Unit Price".to_string(),
    "Unit Price Tax".to_string(),
    "Shipping Charge".to_string(),
    "Total Discounts".to_string(),
    "Total Owed".to_string(),
    "Shipment Item Subtotal".to_string(),
    "Shipment Item Subtotal Tax".to_string(),
    "Quantity".to_string(),
    "Payment Instrument Type".to_string(),
    "Shipping Address".to_string(),
    "Billing Address".to_string(),
    "Product Name".to_string(),
  ];

  // TODO: Is it possible for there to be multiple OrderHistory directories?
  let full_filepath = zip_filepath.to_string() + "/Reatail.OrderHistory.1/Retail.OrderHistory.1.csv";
  let file = extract_file_from_zip(&full_filepath, "Retail.OrderHistory.1.csv").unwrap();
  let reader = std::io::Cursor::new(file);
  let lines = read_csv_from_reader(reader, &pick_headers);

  lines
}
