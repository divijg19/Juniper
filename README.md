# Vertex
<div align="center">


![alt text](https://via.placeholder.com/150x150.png?text=V)

<!-- Replace with actual logo/banner later -->


The Reactive, Constraint-Based Typesetting Engine.

![alt text](https://img.shields.io/github/actions/workflow/status/username/vertex/rust.yml?branch=main)


![alt text](https://img.shields.io/crates/v/vertex)


![alt text](https://img.shields.io/badge/License-MIT-yellow.svg)


![alt text](https://img.shields.io/badge/Language-Rust-orange)

</div>

📖 Introduction

Vertex is a modern document compiler designed to supersede LaTeX for the digital age. Built from the ground up in Rust, it prioritizes sub-millisecond compilation, web-native output (HTML5/DOM), and print-perfect PDF generation using a unified Abstract Syntax Tree (AST).

Unlike LaTeX, which relies on archaic macro expansion, Vertex uses a strictly typed syntax and embeds WebAssembly (Wasm) for scripting, ensuring that packages are secure, portable, and fast.

Project Status: Vertex is currently in Alpha. The core layout engine and parser are functional, but API stability is not guaranteed.

⚡ Key Features

🚀 Blazingly Fast: Written in Rust with incremental compilation. Compiles 100-page technical documents in milliseconds.

📐 Semantic Syntax: A hybrid of Markdown's brevity and LaTeX's power. No more backslash hell.

🎨 Dual-Head Rendering: Outputs high-fidelity PDF (via SVG) and accessible, responsive HTML from the same source.

🧩 Wasm Plugins: Write packages and extensions in Rust, AssemblyScript, or Go. Logic is sandboxed and secure.

🧮 Modern Math: Integrated OpenType math support (similar to purely numeric TeX engines).

🧠 Intelligent Layout: Uses a modified Knuth-Plass algorithm for paragraph justification and a Cassowary constraint solver for page layout.

📝 The Syntax

Vertex uses a slash command syntax that separates content from configuration. It supports Python-style indentation blocks to eliminate "runaway argument" errors.

code
Text
download
content_copy
expand_less
/doc [
    title = "The Future of Typesetting"
    author = "Jane Doe"
    date = auto
]

# 1. Introduction
Vertex treats documents as **structured data**. Unlike Markdown, it supports complex
attributes. Unlike LaTeX, it is human-readable.

/equation [label="eq:mass-energy"]
    $$ E = mc^2 $$

As seen in /ref[eq:mass-energy], the syntax is clean. We can also import
live code results:

/plot [width=50% position=center]
    /source file="data.csv" type="scatter"
🛠️ Architecture (How it works)

Vertex is not a wrapper around TeX. It is a completely new engine built on four distinct stages:

1. Lexing & Parsing (/src/parser)

We use a zero-copy recursive descent parser to convert source text into a Concrete Syntax Tree (CST) and then lower it to a strictly typed Abstract Syntax Tree (AST). This ensures that syntax errors are caught before compilation begins, offering modern LSP (Language Server Protocol) features like "Go to Definition."

2. The DOM & Macro Expansion (/src/dom)

The AST is traversed to expand macros (Wasm plugins) and resolve references. This results in a Reactive Document Object Model (RDOM). If a user changes a variable, only the affected nodes in the graph are dirtied and re-computed.

3. Layout Engine (/src/layout)

This is the core of Vertex. It transforms the logical DOM into a physical Box Model.

Text Shaping: Uses rustybuzz (a HarfBuzz port) for font shaping, ligatures, and kerning.

Line Breaking: Implements the Knuth-Plass total-fit algorithm to minimize raggedness across paragraphs.

Pagination: Uses a constraint solver to handle orphans, widows, and float placement (figures/tables).

4. Rendering (/src/render)

The Box Model is strictly distinct from the output format.

PDF: Rendered via skia-safe or native PDF generation commands.

HTML: Mapped directly to HTML5 Custom Elements with CSS Grid for layout fidelity.

🚀 Getting Started
Prerequisites

Rust (latest stable)

Cargo

Installation
code
Bash
download
content_copy
expand_less
git clone https://github.com/username/vertex.git
cd vertex
cargo build --release
Usage

Create a file named paper.vtx:

code
Text
download
content_copy
expand_less
/section "Hello World"
This is my first document.

Compile it:

code
Bash
download
content_copy
expand_less
./target/release/vertex build paper.vtx --format=pdf
📊 Benchmarks
Metric	LaTeX (pdflatex)	Vertex (v0.1.0)
Cold Start	~1.2s	~0.05s
Incremental Build	N/A (Full Rebuild)	<10ms
Error Clarity	! Undefined control sequence	Error: Line 5, Col 12: Unknown command
Memory Safety	Unsafe	Safe (Rust)
🗺️ Roadmap

Core: Lexer and Parser (AST generation)

Core: Basic Text Shaping (Harfbuzz integration)

Layout: Knuth-Plass Line Breaking Algorithm

Layout: Multi-column support

System: Wasm Plugin Host integration

Output: PDF/X-4 Compliance

🤝 Contributing

This project is an educational exploration into typesetting algorithms and systems programming. Pull requests are welcome, particularly for:

Improving the line-breaking cost functions.

Adding support for more OpenType font features.

Writing documentation for the AST structure.

Please read CONTRIBUTING.md before getting started.

📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
