# 👻 GhostByte

### Binary Intelligence Terminal for Deterministic Payload Engineering

**Team Name:** Krishnap

**Hackathon:** Hackathon Raptors

---

## 🎯 Assigned Constraints

| Constraint | Assignment |
|------------|------------|
| D1 | Short-Name Ninja |
| D2 | Detailed Creator (300 LOC Budget) |
| D3 | System Utilities |
| D4 | Rust |

### How GhostByte Satisfies Them

✅ **Short-Name Ninja**

All project-defined identifiers were limited to three characters or fewer.

Examples:

```text
Cmd
Ftr
cal
prs
siz
off
nln
sts
hos
fil

**Bonus Challenges Completed:** ✅ Rosetta Stone (+5) | ✅ Cross-Constraint Combo (+5) | ✅ Language Love Letter (+3)
```

---

# 🚀 Overview

GhostByte is a compact systems utility that implants, inspects, and extracts payloads from existing files using a deterministic append-only binary protocol.

Unlike conventional file modification tools, GhostByte never alters the original host bytes. Instead, it appends structured metadata and payload information to the end of a binary, enabling deterministic reconstruction through reverse footer traversal.

GhostByte transforms binary files into self-describing containers while preserving original functionality.

---

# 🎥 Demo Video

**Watch the complete project demonstration here:**

# 🎥 Demo Video

<p align="center">
  <a href="https://youtu.be/FBG8uRyKKEA">
   <img alt="GhostByte Demo Video" width="900" src="https://github.com/user-attachments/assets/c83cdb4f-0b9f-4d6f-bca0-308eaa8aca8f" />
  </a>
</p>

<p align="center">
  <b>▶ Click the thumbnail above to watch the full demo</b>
</p>

The demo covers:

* Project architecture walkthrough
* Rust implementation
* Payload implantation
* Binary inspection dashboard
* Payload extraction
* Python reference implementation
* Bonus challenge showcase

---

# 🏆 Why GhostByte?

Modern tooling often hides binary complexity behind frameworks and abstractions.

GhostByte intentionally moves in the opposite direction.

Every byte matters.

Every offset is explicit.

Every transformation is reversible.

The project demonstrates how a deterministic binary protocol can be built under aggressive implementation constraints while remaining portable and transparent.

---

# ✨ Features

### Implant Payloads

Embed arbitrary files into host binaries.

```bash
cargo run --release -- implant secret.txt demo.pdf
```

---

### Binary Intelligence Dashboard

Inspect embedded metadata through a custom terminal interface.

```bash
cargo run --release -- inspect demo.pdf
```

Displays:

* Host information
* Integrity verification
* Embedded payload details
* Payload preview
* Binary layout visualization

---

### Extract Payloads

Recover embedded files through deterministic reverse parsing.

```bash
cargo run --release -- extract demo.pdf
```

---

### Portable Protocol

A second implementation was developed in Python to demonstrate protocol portability.

---

# 📂 Binary Layout

GhostByte transforms files into:

```text
[ HOST ]
[ PAYLOAD ]
[ PAYLOAD NAME ]
[ FOOTER ]
[ GHOSTBYTEv1 ]
```

The original host bytes remain untouched.

GhostByte only appends deterministic regions.

---

# 🔧 Footer Protocol

Every embedded payload contains deterministic metadata.

| Field        | Type |
| ------------ | ---- |
| Version      | u8   |
| Payload Size | u64  |
| Checksum     | u32  |
| Name Length  | u16  |
| Offset       | u64  |
| Flags        | u8   |

Footer Size:

```text
24 bytes
```

---

# 🧠 Parsing Strategy

GhostByte performs deterministic reverse traversal.

```text
EOF
 ↓
MAGIC
 ↓
FOOTER
 ↓
NAME
 ↓
PAYLOAD
```

Workflow:

1. Locate EOF
2. Validate magic signature
3. Parse footer
4. Recover offsets
5. Verify checksum
6. Reconstruct payload

No scanning.

No heuristics.

No ambiguity.

---

# 🖥️ Demo Screenshots

## Screenshot 1 — Payload Implantation

Demonstrates append-only payload implantation into a host PDF while preserving original bytes.

<img width="1919" height="1074" alt="image" src="https://github.com/user-attachments/assets/43c6b2e9-df29-4a90-8a26-9e996bc0dfaf" />


---

## Screenshot 2 — Binary Intelligence Dashboard

GhostByte reverse-parses the binary structure and visualizes embedded metadata, integrity status, payload information, and protocol layout.

<img width="1919" height="1079" alt="image" src="https://github.com/user-attachments/assets/6369e134-e038-4a72-aa2b-c027ea09fa29" />


---

## Screenshot 3 — Payload Extraction

Deterministic reconstruction of the embedded payload using reverse footer traversal.

<img width="1919" height="1073" alt="image" src="https://github.com/user-attachments/assets/19f92667-c33b-4255-85a8-78b0676736d7" />


---

## Screenshot 4 — Python Rosetta Stone Implementation

Independent Python implementation of the GhostByte protocol demonstrating cross-language portability.

<img width="1919" height="1079" alt="image" src="https://github.com/user-attachments/assets/8f4dcf97-3e02-4dc8-803a-f1038d76d59c" />


---

## Screenshot 5 — Python Payload Recovery

Payload extraction using the Python reference implementation.

<img width="1919" height="301" alt="image" src="https://github.com/user-attachments/assets/3f289571-a40d-401c-878f-2e1b184fe35e" />


---

# 🏗️ Architecture

```text
             GhostByte

        ┌──────────────┐
        │   IMPLANT    │
        └──────┬───────┘
               │
               ▼

[HOST][PAYLOAD][NAME][FOOTER][MAGIC]

               ▲
               │

        ┌──────┴───────┐
        │   INSPECT    │
        └──────┬───────┘
               │
               ▼

        ┌──────────────┐
        │   EXTRACT    │
        └──────────────┘
```

---

# ⚡ Constraint Compliance

## D1 — Short-Name Ninja

All project-defined identifiers were constrained to three characters or fewer.

Examples:

```text
Cmd
Ftr
cal
prs
siz
off
nln
sts
prv
hos
fil
```

This forced concise protocol-oriented naming across the entire codebase.

---

## D2 — Detailed Creator (300 LOC)

The implementation was intentionally engineered to remain under the strict line budget.

This forced:

* Minimal abstractions
* Compact protocol design
* Focused functionality
* High architectural density

Every line had to justify its existence.

---

## D3 — System Utilities

GhostByte operates as a low-level binary systems utility.

Capabilities include:

* Binary augmentation
* Payload reconstruction
* Integrity verification
* Protocol inspection
* Metadata visualization

---

## D4 — Rust

Rust provided:

* Ownership-safe file handling
* Deterministic byte manipulation
* Explicit buffer management
* Structured binary parsing

The resulting implementation remains compact, predictable, and safe.

---

# 🏆 Bonus Challenge #1 — Cross-Constraint Combo (+5)

The most interesting design emerged from the collision between:

```text
Short-Name Ninja
+
300 LOC Budget
```

The naming constraint compressed the vocabulary.

The line-budget constraint compressed the architecture.

Together they produced a protocol-oriented utility where:

* APIs became minimal
* Parsing became deterministic
* Structures became compact
* Visualization remained lightweight

The constraints did not merely restrict implementation.

They actively created the final architecture.

Without those constraints GhostByte would likely have evolved into a larger and more abstract system.

Instead, they produced a complete binary engineering utility inside a highly compressed implementation footprint.

---

# 🏆 Bonus Challenge #2 — Rosetta Stone (+5)

GhostByte is intentionally protocol-oriented.

The binary format is language-independent.

To prove this, the protocol was independently reimplemented in Python.

Directory:

```text
python_reference/
└── ghostbyte.py
```

Both implementations support:

* Implant
* Inspect
* Extract
* Footer Parsing
* Checksum Verification

This demonstrates that GhostByte is not merely a Rust application.

It is a portable binary protocol that can be implemented across ecosystems.

---

# 🏆 Bonus Challenge #3 — Language Love Letter (+3)

One surprising lesson from Rust was how naturally ownership encourages append-only workflows.

Initially, modifying host files directly seemed simpler.

However, Rust's ownership model encouraged explicit data flow:

```text
Host
 ↓
Payload
 ↓
Footer
 ↓
Magic
```

Rather than mutating existing bytes, GhostByte evolved into a cleaner append-only architecture.

The language constraints directly improved the final design.

The project became safer, easier to reason about, and more deterministic because of Rust's ownership philosophy.

---

# 📁 Project Structure

```text
ghostbyte/
├── python_reference/
│   └── ghostbyte.py
│
├── samples/
│   ├── demo.pdf
│   └── secret.txt
│
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── checksum.rs
│   ├── footer.rs
│   ├── implant.rs
│   ├── extract.rs
│   └── inspect.rs
│
├── Cargo.toml
├── Cargo.lock
├── README.md
└── .gitignore
```

---

# 🔨 Build

```bash
cargo build --release
```

---

# ▶️ Complete Workflow

## Implant

```bash
cargo run --release -- implant secret.txt demo.pdf
```

## Inspect

```bash
cargo run --release -- inspect demo.pdf
```

## Extract

```bash
cargo run --release -- extract demo.pdf
```

## Python Reference

```bash
cd python_reference

python ghostbyte.py implant ../samples/secret.txt ../samples/demo.pdf

python ghostbyte.py inspect ../samples/demo.pdf

python ghostbyte.py extract ../samples/demo.pdf
```

---

# 🎯 Key Takeaways

GhostByte demonstrates how:

* Binary engineering
* Protocol design
* Integrity verification
* Terminal visualization
* Cross-language portability
* Constraint-driven development

can coexist inside a compact systems utility.

The project intentionally favors:

* Transparency
* Determinism
* Reversibility
* Portability
* Simplicity

while remaining small enough to satisfy aggressive implementation constraints.

GhostByte ultimately evolved into a Binary Intelligence Terminal rather than a traditional command-line utility, proving that constraints can generate architecture rather than merely limit it.

---

**Built with Rust 🦀 by Team Krishnap for Hackathon Raptors.**
