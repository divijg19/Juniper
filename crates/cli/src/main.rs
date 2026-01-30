use std::env;
use std::fs;
use std::io::{self, Read};

use juniper_dom::build_rdom;
use juniper_layout::layout;
use juniper_parser::parse_source;
use juniper_render::{HtmlRenderer, PrintRenderer, Renderer};

/// CLI entrypoint: reads input from the first argument path, or stdin if `-` or
/// no argument is provided, runs the pipeline and prints a small render.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let mut src = String::new();
    if let Some(path) = args.next() {
        if path == "-" {
            io::stdin().read_to_string(&mut src)?;
        } else {
            src = fs::read_to_string(path)?;
        }
    } else {
        // default demo text
        src = String::from("hello juniper world");
    }

    let ast = parse_source(&src).map_err(|e| format!("parse failed: {}", e))?;
    let rdom = build_rdom(&ast);
    let doc = layout(&rdom);

    // print text renderer
    let printer = PrintRenderer;
    printer.render(&doc)?;

    // HTML output (also printed to stdout)
    let html = HtmlRenderer;
    html.render(&doc)?;

    Ok(())
}
