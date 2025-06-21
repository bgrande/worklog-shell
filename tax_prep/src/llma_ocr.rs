use anyhow::Result as AnyResult;
use llama_cpp_rs::{LLama, LLamaParameters};
use serde::{Deserialize, Serialize};
use tesseract::Tesseract;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractedInvoiceData {
    total_amount: Option<f64>,
    net_amount: Option<f64>,
    vat_amount: Option<f64>,
    vat_rate: Option<f64>,
    invoice_number: Option<String>,
    invoice_date: Option<String>,
    payment_terms: Option<String>,
    supplier: Option<String>,
}

pub struct InvoiceExtractor {
    model: LLama,
    prompt_template: String,
}

impl InvoiceExtractor {
    pub fn new(model_path: &str) -> AnyResult<Self> {
        let params = LLamaParameters {
            model_path: model_path.into(),
            context_size: 2048,
            batch_size: 512,
            ..Default::default()
        };

        let model = LLama::new(params)?;

        // Systemanweisung und Prompt-Template für strukturierte Extraktion
        let prompt_template = r#"
Du bist ein Experte für die Analyse von Rechnungsdokumenten.
Extrahiere die folgenden Informationen aus dem Text und gib sie im JSON-Format zurück.
Verwende nur Informationen, die im Text eindeutig zu finden sind.
Wenn eine Information nicht gefunden wird, setze null.

Benötigte Informationen:
- Gesamtbetrag (total_amount)
- Nettobetrag (net_amount)
- Mehrwertsteuerbetrag (vat_amount)
- Mehrwertsteuersatz (vat_rate)
- Rechnungsnummer (invoice_number)
- Rechnungsdatum (invoice_date)
- Zahlungsbedingungen (payment_terms)
- Lieferant (supplier)

Text der Rechnung:
{text}

Antworte nur mit einem validen JSON-Objekt.
"#.to_string();

        Ok(Self {
            model,
            prompt_template,
        })
    }

    pub fn extract_from_image(&self, image_path: &str) -> AnyResult<ExtractedInvoiceData> {
        // OCR durchführen
        let text = self.perform_ocr(image_path)?;

        // Text durch LLM verarbeiten
        let extracted_data = self.process_with_llm(&text)?;

        Ok(extracted_data)
    }

    fn perform_ocr(&self, image_path: &str) -> AnyResult<String> {
        let mut tess = Tesseract::new(None, Some("deu"))?;
        tess.set_image_from_path(image_path)?;
        Ok(tess.get_text()?)
    }

    fn process_with_llm(&self, text: &str) -> AnyResult<ExtractedInvoiceData> {
        // Prompt mit aktuellem Text erstellen
        let prompt = self.prompt_template.replace("{text}", text);

        // LLM-Inferenz durchführen
        let response = self.model.inference(
            &prompt,
            256,  // max_tokens
            0.7,  // temperature
            0.95, // top_p
        )?;

        // Versuche die Antwort als JSON zu parsen
        let data: ExtractedInvoiceData = serde_json::from_str(&response)
            .map_err(|e| anyhow::anyhow!("Fehler beim JSON-Parsing: {}", e))?;

        Ok(data)
    }
}