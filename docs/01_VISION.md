# Juniper — Vision

## Why Juniper Exists

Juniper exists because modern documents are no longer static text artifacts—they are **structured, reactive systems**—and existing typesetting tools were not designed for this reality.

LaTeX was created in an era of batch compilation, macro expansion, and print-only output. While extraordinarily influential, its core abstractions are fundamentally mismatched with:
- real-time feedback
- incremental compilation
- secure extensibility
- web-native rendering
- modern developer tooling

Juniper does not attempt to fix LaTeX.
It replaces its assumptions entirely.

---

## The Core Thesis

> **Documents are programs—but they should be safe, deterministic, and declarative.**

Juniper treats documents as:
- **typed data structures**, not token streams
- **reactive graphs**, not linear passes
- **constraint-solved layouts**, not imperative hacks
- **multi-target outputs**, not PDF-first artifacts

Every design decision in Juniper follows from this thesis.

---

## What Juniper Is (and Is Not)

### Juniper Is:
- A **document compiler**, not a markup preprocessor
- A **layout engine**, not a style wrapper
- A **deterministic system**, not a macro playground
- A **platform**, not a single-format tool

### Juniper Is Not:
- A TeX derivative
- A macro language
- A scripting runtime with formatting bolted on
- A browser-first toy renderer

---

## Why LaTeX Cannot Be Fixed

LaTeX’s limitations are not accidental—they are architectural:

- Macro expansion occurs before structure is known
- Errors cascade unpredictably
- Layout decisions are global and opaque
- Extensibility is unrestricted and unsafe
- Incremental compilation is impossible by design

Any attempt to modernize LaTeX must either:
1. Break compatibility, or
2. Preserve broken abstractions

Juniper chooses neither.  
It starts over.

---

## Juniper’s Architectural Commitments

Juniper is built around several non-negotiable commitments:

### 1. **Structure Before Layout**
Parsing produces a **typed AST** before any layout occurs.
Errors are structural, localized, and deterministic.

### 2. **Layout as a Solved Problem**
Layout is expressed as:
- optimization (line breaking)
- constraints (pagination, floats)
- geometry (box trees)

—not procedural instructions.

### 3. **Reactive by Default**
Documents form a dependency graph.
Edits invalidate only affected nodes.
Recompilation is incremental and predictable.

### 4. **Format-Agnostic Output**
PDF and HTML are targets—not drivers.
Layout decisions are independent of rendering backends.

### 5. **Secure Extensibility**
All extensions execute inside sandboxed WebAssembly.
No arbitrary IO.
No hidden side effects.
No privilege escalation.

---

## The Role of Rust

Rust is not incidental—it is essential.

Rust enables Juniper to guarantee:
- memory safety
- deterministic behavior
- high-performance incremental computation
- clear ownership boundaries across stages

Juniper’s correctness model relies on Rust’s type system as much as its runtime speed.

---

## Comparison to Contemporary Systems

Juniper acknowledges modern efforts such as Typst and Pandoc, but differs fundamentally in scope and intent:

- Juniper prioritizes **layout research** over syntax innovation
- Juniper treats **HTML as a first-class output**, not a secondary export
- Juniper enforces **strict separation** between content, structure, layout, and rendering
- Juniper is designed for **long-lived documents**, not only rapid authoring

Juniper is slower to design—but faster to evolve correctly.

---

## Long-Term Direction

Juniper is designed to support:
- academic publishing
- technical documentation
- scientific papers
- interactive and web-native documents
- tooling integration (LSPs, CI, automated builds)

It intentionally avoids:
- presentation tooling
- WYSIWYG-first workflows
- ad-hoc scripting embedded in content

---

## Design Philosophy

Juniper follows a single guiding principle:

> **Make the correct model unavoidable, and the incorrect model impossible.**

This applies to:
- syntax
- layout decisions
- plugin behavior
- error handling
- rendering semantics

Juniper is not optimized for shortcuts.
It is optimized for correctness, clarity, and longevity.

---

## Status and Expectations

Juniper is in **early alpha**.

This project values:
- architectural soundness over velocity
- explicit design over convenience
- long-term maintainability over feature count

Juniper is not yet stable.
It is, however, deliberate.

---

## Closing Statement

Juniper is an attempt to answer a simple question honestly:

> *If we were to design a typesetting engine today—knowing everything we now know—what would it look like?*

This repository is that answer, in progress.
