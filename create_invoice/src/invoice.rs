use chrono::{Datelike, Local, NaiveDateTime};
use crate::structs::{Config, Invoice, Position};
use anyhow::Result;
use rand::Rng;

pub(crate) fn create_invoice(positions: Vec<Position>, customer_number: &str, config: &Config) -> Result<Invoice> {
    let now = Local::now().naive_local();
    let mut total_net: f64 = 0.0;
    let mut total_gross: f64 = 0.0;
    let mut vat_id = String::new();
    let mut vat_value: f64 = 0.0;
    let mut execution_dates: Vec<Option<NaiveDateTime>> = Vec::new();
    let mut delivery_dates: Vec<Option<NaiveDateTime>> = Vec::new();

    // Calculate totals
    for position in &positions {
        for detail in &position.positions {
            // Parse net price from string
            let net_price: f64 = detail.net_price.parse()?;
            let position_total = detail.amount * net_price;
            total_net += position_total;
            
            execution_dates.push(detail.date_execution);
            delivery_dates.push(detail.date_delivery);

            // Get VAT rate from config
            if let Some(tax) = config.taxes.get(&detail.vat_id) {
                let vat_rate: f64 = tax.value.parse()?;
                total_gross += position_total * (1.0 + vat_rate);

                // Store VAT info from first position (assuming all positions use same VAT)
                if vat_id.is_empty() {
                    vat_id = detail.vat_id.clone();
                    vat_value = vat_rate;
                }
            }
        }
    }

    // Get latest execution date
    let latest_execution_date: Option<NaiveDateTime> = execution_dates.iter()
        .filter_map(|&date| date)  // Remove None values
        .max();  // Get the latest date

    // Get latest delivery date
    let latest_delivery_date: Option<NaiveDateTime> = delivery_dates.iter()
        .filter_map(|&date| date)
        .max();



    Ok(Invoice {
        id: generate_invoice_id(),
        customer_id: positions.first().map_or_else(|| "".to_string(), |p| p.customer_id.clone()),
        date_created: now,
        date_execution: latest_execution_date,
        date_delivery: latest_delivery_date,
        customer_number: customer_number.to_string(),
        positions,
        total_net_price: format!("{:.2}", total_net).replace(".", ","),
        total_gross_price: format!("{:.2}", total_gross).replace(".", ","),
        vat_id,
        vat_value,
        currency: "EUR".to_string(),
        payment_text: None,
        payment_qr: None
    })
}

fn generate_invoice_id() -> String {
    let now = Local::now().naive_local();
    let year = now.year();
    let random_number: u32 = rand::rng().random_range(10000..100000);
    format!("{}_{:05}", year, random_number)
}

