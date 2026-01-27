use juniper_dom::build_rdom;
use juniper_layout::layout;
use juniper_parser::parse_source;
use juniper_render::{PrintRenderer, Renderer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let src = "hello juniper world";
    let ast = parse_source(src).expect("parse failed");
    let rdom = build_rdom(&ast);
    let doc = layout(&rdom);
    let renderer = PrintRenderer;
    renderer.render(&doc)?;
    Ok(())
}
