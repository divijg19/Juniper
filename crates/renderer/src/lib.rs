use juniper_layout::PhysicalDoc;

pub trait Renderer {
    fn render(&self, doc: &PhysicalDoc) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct PrintRenderer;

impl Renderer for PrintRenderer {
    fn render(&self, doc: &PhysicalDoc) -> Result<(), Box<dyn std::error::Error>> {
        for b in &doc.boxes {
            println!(
                "BOX @({:.1},{:.1}) {:.1}x{:.1}: {}",
                b.x, b.y, b.w, b.h, b.content
            );
        }
        Ok(())
    }
}

pub fn version() -> &'static str {
    "0.1.0"
}
