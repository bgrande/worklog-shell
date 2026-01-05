mod structs;
mod files;
mod invoice;
mod pdf;

use anyhow::Result as AnyResult;
use chrono::Datelike;
use log::{debug, error};
use std::{env, fs};
use crate::files::read_positions_until_date;
use crate::structs::Config;
use crate::invoice::create_invoice;
use crate::pdf::generate_invoice_pdf;

const DATA_BASE_PATH: &str = "data";
const WORKDIR_BASE_PATH: &str = "workdir";

fn main() -> AnyResult<()> {
    env_logger::init();

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <customer_name>", args[0]);
        return Ok(());
    }

    debug!("Processing with arguments: {:?}", args);

    let customer_name = &args[1];
    let date = chrono::Local::now().naive_local().date();
    let year = date.year();
    //date.format("%Y-%m-%d").to_string();

    let current_dir = match env::current_dir() {
        Err(err) => {
            error!("couldn't get current directory: {}", err);
            return Ok(());
        }
        Ok(dir) => dir.as_path().display().to_string(),
    };

    let dir = fs::read_dir(format!(
        "{}/{}/{}",
        DATA_BASE_PATH, current_dir, customer_name
    ));

    let positions = match read_positions_until_date(
        DATA_BASE_PATH,
        customer_name,
        &date
    ) {
        Ok(positions) => positions,
        Err(err) => {
            error!("couldn't read positions: {}", err);
            return Ok(());
        }
    };

    if positions.is_empty() {
        debug!("No positions found for customer {}", customer_name);
        return Ok(());
    }

    let config = load_config("config.json")?;
    let invoice = create_invoice(positions, "from_customer", &config)?;

    // Save invoice
    let invoice_path = format!(
        "{}/customers/{}/invoices/{}.json",
        DATA_BASE_PATH,
        customer_name,
        date.format("%Y-%m-%d")
    );

    let invoice_json = serde_json::to_string_pretty(&invoice)?;
    fs::write(&invoice_path, invoice_json)?;

    let template_path = "dist/templates/invoice-template.typ";
    let output_path = format!(
        "{}/customers/{}/invoices/{}",
        DATA_BASE_PATH,
        customer_name,
        date.format("%Y-%m-%d")
    );

    generate_invoice_pdf(&invoice_path, "config.json", template_path, &output_path)?;

    Ok(())
}

fn load_config(config_path: &str) -> AnyResult<Config> {
    let config_content = fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&config_content)?;
    Ok(config)
}

