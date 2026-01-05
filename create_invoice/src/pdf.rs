use anyhow::Result as AnyResult;
use std::process::Command;

pub fn generate_invoice_pdf(json_path: &str, config_path: &str, template_path: &str, output_path: &str) -> AnyResult<()> {
    // Erstelle temporäre Typst-Hauptdatei
    let content = format!(
        "#import \"{}\" : *\n\
         #render-invoice(\"{}\", \"{}\")",
        template_path, json_path, config_path
    );

    let temp_typst_path = format!("{}.typ", output_path);
    std::fs::write(&temp_typst_path, content)?;

    // Kompiliere zu PDF
    let output = Command::new("typst")
        .arg("compile")
        .arg(&temp_typst_path)
        .arg(format!("{}.pdf", output_path))
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Typst compilation failed: {}", 
            String::from_utf8_lossy(&output.stderr)));
    }

    // Lösche temporäre Datei
    std::fs::remove_file(temp_typst_path)?;

    Ok(())
}
