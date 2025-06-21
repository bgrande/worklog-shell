use std::option::Option;
use anyhow::Result as AnyResult;
use regex::Regex;
use std::path::Path;
use tesseract::Tesseract;

pub struct InvoiceData {
    pub total_amount: Option<f64>,
    pub net_amount: Option<f64>,
    pub tax_amount: Option<f64>,
    pub tax_rate: Option<f64>,
    pub invoice_number: Option<String>,
    pub date: Option<String>,
}

pub fn extract_invoice_data<P: AsRef<Path>>(image_path: P) -> AnyResult<InvoiceData> {
    let file = match image_path.as_ref().file_name() {
        Some(file_name) => match file_name.to_str() {
            Some(file_name) => file_name,
            None => return Err(anyhow::anyhow!("Invalid image path")),       
        },
        None => return Err(anyhow::anyhow!("Invalid image path")),
    };
    
    let mut tess = Tesseract::new(Some(file), Some("deu"))?;

    // Setze die Bildvorverarbeitung für beste OCR-Ergebnisse
    tess = tess.set_variable("tessedit_char_whitelist", "0123456789.,€$ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-")?;

    // Extrahiere Text
    let text = tess.get_text()?;

    // Parse den extrahierten Text
    parse_invoice_text(&text)
}

fn parse_invoice_text(text: &str) -> AnyResult<InvoiceData> {
    // Reguläre Ausdrücke für verschiedene Felder
    let amount_re = Regex::new(r"(?i)Gesamt\s*betrag\s*[:€]\s*(\d+[.,]\d{2})")?;
    let net_re = Regex::new(r"(?i)Netto\s*betrag\s*[:€]\s*(\d+[.,]\d{2})")?;
    let tax_re = Regex::new(r"(?i)(?:USt|MwSt|Mehrwertsteuer)(?:\s*\(?\s*(\d{1,2})[.,]?\d*\s*%\)?)?[\s:]*(\d+[.,]\d{2})")?;
    let invoice_nr_re = Regex::new(r"(?i)Rechnung(?:s-?(?:nummer|nr))?\.?\s*:?\s*([A-Za-z0-9-]+)")?;
    let date_re = Regex::new(r"(?i)(?:Rechnungs?|Datum)\s*:?\s*(\d{1,2}[./-]\d{1,2}[./-]\d{2,4})")?;

    let mut data = InvoiceData {
        total_amount: None,
        net_amount: None,
        tax_amount: None,
        tax_rate: None,
        invoice_number: None,
        date: None,
    };

    // Extrahiere Werte mit den RegEx-Patterns
    if let Some(cap) = amount_re.captures(text) {
        data.total_amount = match cap.get(1)
            .map(|m| parse_amount(m.as_str())) {
            Some(Some(amount)) => Some(amount),
            _ => None,       
        };
    }

    if let Some(cap) = net_re.captures(text) {
        data.net_amount = match cap.get(1)
            .map(|m| parse_amount(m.as_str())) {
            Some(Some(amount)) => Some(amount),
            _ => None,
        };
    }

    if let Some(cap) = tax_re.captures(text) {
        data.tax_rate = cap.get(1)
            .map(|m| m.as_str().parse::<f64>().ok())
            .flatten();
        data.tax_amount = match cap.get(2)
            .map(|m| parse_amount(m.as_str())) {
            Some(Some(amount)) => Some(amount),
            _ => None,
        };
    }

    data.invoice_number = invoice_nr_re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string());

    data.date = date_re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string());

    Ok(data)
}

fn parse_amount(amount_str: &str) -> Option<f64> {
    amount_str
        .replace(".", "")
        .replace(",", ".")
        .parse::<f64>()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_invoice_text() {
        let text = r#"
            Rechnung Nr.: 2024-001
            Datum: 22.03.2024
            
            Nettobetrag: 100,00
            MwSt. (19%): 19,00
            Gesamtbetrag: 119,00 EUR
        "#;

        let result = parse_invoice_text(text).unwrap();
        assert_eq!(result.total_amount, Some(119.0));
        assert_eq!(result.net_amount, Some(100.0));
        assert_eq!(result.tax_amount, Some(19.0));
        assert_eq!(result.tax_rate, Some(19.0));
        assert_eq!(result.invoice_number, Some("2024-001".to_string()));
        assert_eq!(result.date, Some("22.03.2024".to_string()));
    }
}