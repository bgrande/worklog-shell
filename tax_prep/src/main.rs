//mod ocr;
//mod llma_ocr;
//mod image_optimization;
mod parse;

use std::{env, fs};
use std::path::Path;
use anyhow::Result as AnyResult;
use log::debug;
use crate::parse::parse_invoice_text;
//use crate::image_optimization::ImagePreprocessor;
//use crate::llma_ocr::InvoiceExtractor;
//use crate::ocr::extract_invoice_data;

fn main() -> AnyResult<()> {
    env_logger::init();

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path/to/file_name.txt>", args[0]);
        return Ok(());
    }

    debug!("Processing with arguments: {:?}", args);

    let file_name = &args[1];

    let path = Path::new(file_name);
    let basename= match path.file_stem() {
        Some(stem) => match stem.to_str() {
            Some(str) => str,
            None => "unknown"
        },
        None => "unknown"
    };

    /*
    let preprocessor = ImagePreprocessor::new();
    let processed_img = preprocessor.process_image(file_name)?;
    let processed_file_name = format!("{}_processed.tiff", basename);
    processed_img.save(&processed_file_name)?;
    
    let invoice_data = extract_invoice_data(processed_file_name)?;
     */

    let file_text = fs::read_to_string(file_name)?;
    let invoice_data = parse_invoice_text(&file_text)?;

    println!("Extrahierte Daten:");
    if let Some(amount) = invoice_data.total_amount {
        println!("Gesamtbetrag: {:.2} EUR", amount);
    }
    if let Some(net) = invoice_data.net_amount {
        println!("Nettobetrag: {:.2} EUR", net);
    }
    if let Some(tax) = invoice_data.tax_amount {
        println!("Steuerbetrag: {:.2} EUR", tax);
    }
    if let Some(rate) = invoice_data.tax_rate {
        println!("Steuersatz: {:.1}%", rate);
    }

    // todo write parsed data into json file

    // Initialisiere den Extraktor mit lokalem Modell
    //let extractor = InvoiceExtractor::new("models/mistral-7b-instruct-v0.2.Q4_K_M.gguf")?;

    // Verarbeite eine Rechnung
    //let data = extractor.extract_from_image("path/to/invoice.tiff")?;

    //println!("Extrahierte Daten: {:#?}", data);

    Ok(())
}