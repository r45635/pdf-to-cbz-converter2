pub mod pdf_renderer;
pub mod image_processor;
pub mod archive;
pub mod pdf_creator;

// Note: The following modules are kept for potential future use but are not currently active:
// - pdf_extractor: Direct PDF image extraction (currently using pdfium-render instead)
// - ghostscript_renderer: Ghostscript-based PDF rendering (optional external dependency)
// - pdf_content_analyzer: PDF structure analysis
// - imagemagick_converter: ImageMagick-based conversion (optional external dependency)

pub use pdf_renderer::*;
pub use image_processor::*;
pub use archive::*;
pub use pdf_creator::*;
pub use pdf_conversion_lib::bind_pdfium;
