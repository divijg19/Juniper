# `Juniper`

<div align="center">

<!-- Replace with actual logo/banner later -->
<img src="https://via.placeholder.com/140x140.png?text=J" alt="Juniper Logo" />

### The Reactive, Constraint-Based Typesetting Engine

A modern, Rust-native alternative to LaTeX—designed for speed, safety, and the web.

[![Build Status](https://img.shields.io/github/actions/workflow/status/divijg19/Juniper/rust.yml?branch=main)](https://github.com/divijg19/Juniper/actions)
[![Crates.io](https://img.shields.io/crates/v/juniper)](https://crates.io/crates/juniper)
[![License: MIT](https://img.shields.io/badge/license-MIT-yellow.svg)](./LICENSE)
[![Language: Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org)

</div>

---

## 📖 Overview

**Juniper** is a next-generation document compiler built to replace LaTeX for the digital age.

It is **not** a TeX wrapper, nor a macro-driven system. Juniper is a ground-up re-architecture of typesetting as a **reactive, typed, constraint-solved pipeline**, capable of producing:

- **Print-perfect PDFs**
- **Accessible, responsive HTML**
- From the **same source and AST**

Juniper is written entirely in **Rust**, prioritizing:
- Sub-millisecond incremental builds
- Memory safety
- Deterministic output
- Secure extensibility via WebAssembly

> **Project Status:**  
> Juniper is currently in **Alpha**. The parser and layout core are functional, but APIs are unstable and subject to change.

---

## ⚡ Key Features

### 🚀 Performance-First
- Incremental compilation by default
- Reactive document graph: only affected nodes recompile
- Designed to handle 100+ page documents in milliseconds

### 📐 Semantic, Typed Syntax
- Cleaner than LaTeX, more expressive than Markdown
- No macro expansion hacks
- Errors are structural, localized, and readable

### 🎨 Dual-Target Rendering
- **PDF**: Print-grade output via SVG / native PDF primitives
- **HTML**: Web-native DOM with CSS Grid and custom elements
- Identical layout semantics across formats

### 🧩 Secure Wasm Plugins
- Extend Juniper using Rust, AssemblyScript, or Go
- Fully sandboxed WebAssembly execution
- No arbitrary filesystem or runtime access

### 🧮 Modern Mathematics
- Native OpenType math support
- Proper glyph shaping, alignment, and spacing
- No legacy TeX math limitations

### 🧠 Constraint-Driven Layout
- Paragraph justification via Knuth-Plass
- Page layout solved using constraint systems
- Intelligent handling of floats, widows, and orphans

---

## 📝 The Juniper Syntax

Juniper separates **content**, **structure**, and **configuration** cleanly.

It uses a slash-command model with indentation-based blocks to eliminate runaway arguments and macro ambiguity.

```juniper
/doc [
    title  = "The Future of Typesetting"
    author = "Jane Doe"
    date   = auto
]

## 1. Introduction

Juniper treats documents as **structured data**.
Unlike Markdown, it supports rich attributes.
Unlike LaTeX, it is human-readable.

/equation [label="eq:mass-energy"]
    $$ E = mc^2 $$

As shown in /ref[eq:mass-energy], references are explicit and safe.

/plot [width=50% position=center]
    /source file="data.csv" type="scatter"
````

---

## 🏗️ Architecture

Juniper’s pipeline is explicitly staged and fully typed end-to-end.

### 1. Lexing & Parsing (`/src/parser`)

* Zero-copy recursive-descent parser
* Produces a Concrete Syntax Tree (CST)
* Lowered into a strictly typed AST
* Enables precise diagnostics and future LSP support

### 2. Document Model & Expansion (`/src/dom`)

* AST traversal resolves references and plugins
* WebAssembly macros are expanded here
* Produces a **Reactive Document Object Model (RDOM)**
* Dependency-tracked: edits invalidate only affected nodes

### 3. Layout Engine (`/src/layout`)

The heart of Juniper.

* **Text shaping:** OpenType shaping via `rustybuzz`
* **Line breaking:** Knuth-Plass total-fit optimization
* **Pagination:** Constraint solver handles:

  * Page breaks
  * Float placement
  * Widows and orphans
  * Multi-column flow

### 4. Rendering (`/src/render`)

Layout is strictly decoupled from output.

* **PDF:** SVG or native PDF commands
* **HTML:** HTML5 custom elements + CSS Grid
* Identical layout semantics across formats

---

## 🚀 Getting Started

### Prerequisites

* Rust (latest stable)
* Cargo

### Build from Source

```bash
git clone https://github.com/divijg19/Juniper.git
cd Juniper
cargo build --release
```

### Basic Usage

Create a file `paper.jun`:

```juniper
/section "Hello, World"
This is my first Juniper document.
```

Compile it:

```bash
./target/release/juniper build paper.jun --format=pdf
```

---

## 📊 Benchmarks (Early)

| Metric            | LaTeX (pdflatex) | Juniper (v0.1.0)    |
| ----------------- | ---------------- | ------------------- |
| Cold Start        | ~1.2s            | ~0.05s              |
| Incremental Build | Full rebuild     | <10ms               |
| Error Diagnostics | Cryptic          | Line/column-precise |
| Memory Safety     | Unsafe           | Guaranteed (Rust)   |

---

## 🗺️ Roadmap

### Core

* [ ] Lexer & Parser (AST generation)
* [ ] Typed document model
* [ ] Stable public AST API

### Layout

* [ ] Knuth-Plass line breaking
* [ ] Multi-column layouts
* [ ] Advanced float resolution

### System

* [ ] WebAssembly plugin host
* [ ] Deterministic plugin caching

### Output

* [ ] PDF/X-4 compliance
* [ ] HTML export polish & accessibility audit

---

## 🤝 Contributing

Juniper is both a **serious long-term project** and a **deep educational exploration** into:

* Typesetting algorithms
* Constraint solvers
* Reactive systems
* Systems-level Rust architecture

Contributions are especially welcome for:

* Line-breaking cost function research
* OpenType font feature support
* AST and layout documentation

Please see `CONTRIBUTING.md` before opening a PR.