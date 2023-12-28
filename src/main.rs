use pdfium_render::prelude::*;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let pdfium = Pdfium::default();
    let document = pdfium.load_pdf_from_file(&args[1], None).unwrap();
    let render_config = PdfRenderConfig::new();
    let _ = document.pages().first().unwrap().render_with_config(&render_config);
}
