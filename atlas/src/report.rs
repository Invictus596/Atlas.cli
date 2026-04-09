use genpdf::elements;
use genpdf::fonts;
use genpdf::Element;

/// Generate a PDF report from the provided Markdown-ish text.
pub fn generate_pdf(text: &str, output_path: &str) -> Result<(), String> {
    // Load a font
    // Note: this path might vary by system. For a real app, you'd bundle a font or use a font discovery crate.
    let font_dir = "/usr/share/fonts/truetype/liberation";
    let font_family = fonts::from_files(font_dir, "LiberationSans", None)
        .map_err(|e| format!("Failed to load font: {}", e))?;

    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Project Atlas Enterprise Report");

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    doc.push(elements::Paragraph::new("Project Atlas Analysis Report").styled(genpdf::style::Effect::Bold));
    doc.push(elements::Break::new(1.0));

    for line in text.lines() {
        doc.push(elements::Paragraph::new(line));
    }

    doc.render_to_file(output_path).map_err(|e| format!("Failed to render PDF: {}", e))?;

    Ok(())
}
