# Juniper — Architecture

This document describes the **architectural structure** of Juniper: its core stages, invariants, and boundaries.

It is not an API reference.
It is a map of how the system thinks.

---

## Architectural Overview

Juniper is a **multi-stage document compiler** with a strictly linear data flow and reactive invalidation.

At a high level:

```

Source Text
↓
Concrete Syntax Tree (CST)
↓
Abstract Syntax Tree (AST)
↓
Reactive Document Object Model (RDOM)
↓
Logical Layout Tree
↓
Physical Box Tree
↓
Renderer (PDF | HTML)

```

Each stage:
- consumes a well-defined input
- produces an immutable output
- exposes no side effects beyond its boundary

This structure is non-negotiable.

---

## Core Architectural Principles

### 1. One Direction of Flow

Data flows **forward only**.
No stage mutates or queries a later stage.

This guarantees:
- determinism
- debuggability
- reproducibility
- safe incremental recomputation

---

### 2. Structure Before Semantics

Juniper resolves **structure first**, then **meaning**, then **layout**, then **output**.

Errors are surfaced at the earliest possible stage and never masked downstream.

---

### 3. Immutability as a Default

Intermediate representations are:
- immutable
- persistent
- cheaply shareable

Change is expressed via **replacement**, not mutation.

---

### 4. Reactivity Without Implicit State

Reactivity is explicit and localized.
Dependency edges are tracked deliberately.
There is no global mutable cache.

---

## Stage 1: Parsing

**Location:** `juniper-parser`

### Responsibilities
- Lex source text
- Produce a Concrete Syntax Tree (CST)
- Lower CST into a typed AST

### Invariants
- Parsing never performs layout
- Parsing never executes user code
- AST nodes are fully typed
- All syntax errors are local and structural

### Notes
The CST exists to preserve source fidelity for diagnostics and tooling.
The AST exists to eliminate ambiguity.

---

## Stage 2: Document Expansion

**Location:** `juniper-dom`

### Responsibilities
- Resolve references
- Expand macros (via Wasm)
- Build the Reactive Document Object Model (RDOM)

### Invariants
- Expansion is deterministic
- Plugin execution is sandboxed
- No IO beyond declared capabilities
- Graph dependencies are explicit

### Reactive Model
Each node in the RDOM declares:
- inputs
- outputs
- dependencies

When a node changes, only its dependents are invalidated.

---

## Stage 3: Logical Layout

**Location:** `juniper-layout`

### Responsibilities
- Convert structured content into layout intentions
- Perform line breaking
- Resolve paragraph geometry
- Apply layout constraints

### Invariants
- Layout does not know about output formats
- Text shaping is deterministic
- Layout is expressed as optimization + constraints
- No rendering occurs here

### Algorithms
- Knuth–Plass for line breaking
- Constraint solving for pagination
- Greedy fallbacks only as last resort

---

## Stage 4: Physical Box Tree

**Location:** `juniper-layout`

### Responsibilities
- Produce concrete geometric boxes
- Resolve absolute positions and sizes
- Finalize page breaks

### Invariants
- All geometry is explicit
- Units are normalized
- No further decisions are made after this stage

At this point, the document is fully laid out.

---

## Stage 5: Rendering

**Location:** `juniper-render`

### Responsibilities
- Translate boxes into backend-specific output
- Preserve geometry exactly
- Handle format-specific primitives

### Invariants
- Rendering does not influence layout
- No reflow occurs
- Output backends are interchangeable

### Backends
- PDF (SVG or native)
- HTML (DOM + CSS Grid)

---

## Incremental Compilation Model

Juniper tracks dependencies at the **RDOM level**.

A change may invalidate:
- a single node
- a subtree
- a layout region

But never the entire document unless structurally required.

This enables:
- sub-millisecond incremental builds
- live previews
- deterministic caching

---

## Error Propagation Model

Errors are:
- structural (parser)
- semantic (DOM)
- layout (constraints)
- backend (renderer)

Each error:
- is localized
- halts progression at its stage
- never cascades silently

---

## Crate Boundaries

Juniper is designed for modularization.

| Crate | Responsibility |
|-----|---------------|
| `juniper-parser` | CST → AST |
| `juniper-dom` | RDOM, references, plugins |
| `juniper-layout` | Layout & geometry |
| `juniper-render` | Output backends |
| `juniper-wasm` | Plugin host |
| `juniper-cli` | User-facing interface |

Crates may depend downward—but never upward.

---

## Non-Goals

Juniper explicitly avoids:
- implicit global state
- macro text substitution
- backend-specific layout logic
- runtime scripting without sandboxing
- hidden mutation for performance

---

## Architectural Guarantees

If the architecture is followed correctly, Juniper guarantees:

- deterministic output
- reproducible builds
- predictable performance
- debuggable failures
- long-term maintainability

Breaking architectural boundaries may yield short-term speed—but long-term instability.

---

## Closing Note

Juniper’s architecture is designed to make **incorrect implementations difficult**, not easy.

Contributors are expected to:
- understand the pipeline
- respect stage boundaries
- justify deviations explicitly

This document is the contract that enables Juniper to scale without collapsing under its own complexity.