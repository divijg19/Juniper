use juniper_layout::PhysicalDoc;

/// Renderer trait for backends.
pub trait Renderer {
    fn render(&self, doc: &PhysicalDoc) -> Result<(), Box<dyn std::error::Error>>;
}

/// A trivial renderer that prints boxes to stdout.
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

/// HTML backend: returns a simple HTML document representing boxes.
pub struct HtmlRenderer;

impl HtmlRenderer {
    pub fn render_html(&self, doc: &PhysicalDoc) -> String {
        let mut out = String::new();
        out.push_str("<html><body>\n");
        for b in &doc.boxes {
            out.push_str(&format!(
                "<div style=\"position:relative; left:{:.1}px; top:{:.1}px; width:{:.1}px; height:{:.1}px; border:1px solid #666;\">{}</div>\n",
                b.x, b.y, b.w, b.h, html_escape::encode_text(&b.content)
            ));
        }
        out.push_str("</body></html>\n");
        out
    }
}

impl Renderer for HtmlRenderer {
    fn render(&self, doc: &PhysicalDoc) -> Result<(), Box<dyn std::error::Error>> {
        let html = self.render_html(doc);
        println!("{}", html);
        Ok(())
    }
}

pub fn version() -> &'static str {
    "0.1.0"
}
