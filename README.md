````md
# GhostByte

> Binary Intelligence Terminal for Append-Only Payload Engineering

GhostByte is a compact Rust-based system utility that implants, inspects, and extracts payloads from existing files using a deterministic append-only binary protocol.

Rather than modifying original file contents, GhostByte appends structured metadata and payload information to the end of a host binary, preserving the original file while enabling deterministic reconstruction of embedded content.

The project combines binary engineering, protocol design, integrity verification, terminal visualization, and constraint-driven architecture within an extremely small codebase.

---

# Demo

```text
██████╗ ██╗  ██╗ ██████╗ ███████╗████████╗
██╔════╝ ██║  ██║██╔═══██╗██╔════╝╚══██╔══╝
██║  ███╗███████║██║   ██║███████╗   ██║
██║   ██║██╔══██║██║   ██║╚════██║   ██║
╚██████╔╝██║  ██║╚██████╔╝███████║   ██║
 ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝

        Binary Intelligence Terminal
```

```text
╔══════════════════════════════════════════════╗
║                  GHOSTBYTE                   ║
║         Binary Intelligence Terminal         ║
╚══════════════════════════════════════════════╝

┌──────── SYSTEM STATUS ─────────┐
│ HOST   : demo.pdf             │
│ STATE  : VERIFIED             │
└────────────────────────────────┘

┌──────── PAYLOAD INFO ──────────┐
│ FILE   : secret.txt           │
│ SIZE   : 10 bytes             │
│ OFF    : 1412269              │
│ TEXT   : TOP SECRET           │
└────────────────────────────────┘

┌──────── BINARY MAP ────────────┐
│ ███████████████████ HOST       │
│ ██ PAYLOAD                     │
│ █ FOOTER                       │
│ ■ MAGIC                        │
└────────────────────────────────┘
```

---

# What GhostByte Does

GhostByte provides three core operations:

## Implant

Embed a payload into an existing host file.

```bash
ghostbyte implant secret.txt demo.pdf
```

---

## Inspect

Reverse-parse the file structure and visualize embedded content.

```bash
ghostbyte inspect demo.pdf
```

---

## Extract

Recover the embedded payload.

```bash
ghostbyte extract demo.pdf
```

---

# Binary Layout

GhostByte transforms a file into:

```text
[ HOST ]
[ PAYLOAD ]
[ PAYLOAD NAME ]
[ FOOTER ]
[ GHOSTBYTEv1 ]
```

The original host file remains untouched.

GhostByte only appends deterministic binary regions.

---

# Footer Protocol

GhostByte stores metadata in a deterministic footer.

| Field | Type |
|---------|---------|
| Version | u8 |
| Payload Size | u64 |
| Checksum | u32 |
| Name Length | u16 |
| Offset | u64 |
| Flags | u8 |

Footer Size:

```text
24 bytes
```

---

# Parsing Strategy

GhostByte performs reverse traversal from EOF.

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

# Architecture

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

# Constraint Compliance

## D1 — Short-Name Ninja

All project-defined identifiers are three characters or fewer.

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

This constraint shaped the naming style of the entire codebase.

---

## D2 — Detailed Creator (300 LOC)

The implementation was intentionally engineered to fit within the strict line budget.

This forced:

- compact protocol design
- minimal abstractions
- high code density
- focused functionality

Every feature had to justify its existence.

---

## D3 — System Utilities

GhostByte operates as a binary systems utility.

Capabilities:

- binary augmentation
- payload reconstruction
- integrity verification
- binary visualization
- protocol inspection

---

## D4 — Rust

Rust provided:

- ownership-safe file handling
- deterministic byte manipulation
- explicit buffer management
- structured binary parsing

The resulting implementation remains compact and predictable.

---

# Bonus Challenge: Cross-Constraint Combo (+5)

GhostByte's architecture emerged directly from the collision between:

```text
Short-Name Ninja
+
300 Line Budget
```

The naming constraint compressed the vocabulary.

The line-budget constraint compressed the architecture.

Together they forced a protocol-oriented design where:

- structures became compact
- APIs became minimal
- parsing became deterministic
- visualization remained lightweight

Without these constraints GhostByte would likely have evolved into a larger and more abstract system.

Instead, the constraints produced something unexpected:

```text
A complete binary protocol utility
inside a highly compressed architecture.
```

The constraints did not merely restrict implementation.

They actively created the final design.

---

# Bonus Challenge: Rosetta Stone (+5)

GhostByte is intentionally protocol-oriented.

The binary format is language-independent.

To demonstrate protocol portability, the same GhostByte protocol was implemented in Python.

Included Reference:

```text
python_reference/
└── ghostbyte.py
```

Both implementations support:

- payload implantation
- binary inspection
- payload extraction
- footer parsing
- checksum verification

This demonstrates that GhostByte is not merely a Rust application.

It is a portable binary protocol that can be implemented across ecosystems.

---

# Bonus Challenge: Language Love Letter (+3)

One surprising lesson from Rust was how naturally ownership encourages append-only workflows.

Initially, it seemed easier to mutate files directly.

However, Rust's ownership model pushed the design toward explicit data flow:

```text
Host
 ↓
Payload
 ↓
Footer
 ↓
Magic
```

Rather than modifying existing bytes, GhostByte evolved into an append-only architecture.

This ended up producing a cleaner protocol, simpler parsing logic, and safer file handling.

The final design was better because of the language constraints rather than despite them.

---

# Project Structure

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

# Build

```bash
cargo build --release
```

---

# Sample Workflow

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

---

# Example Session

```bash
cargo build --release

cargo run --release -- implant secret.txt demo.pdf

cargo run --release -- inspect demo.pdf

cargo run --release -- extract demo.pdf
```

---

# Why GhostByte?

GhostByte demonstrates how:

- binary engineering
- protocol design
- integrity verification
- terminal visualization
- constraint-driven development

can coexist inside a highly constrained Rust codebase.

The project intentionally favors:

- transparency
- determinism
- reversibility
- portability
- simplicity

while remaining compact enough to satisfy aggressive implementation constraints.

GhostByte ultimately evolved into a Binary Intelligence Terminal rather than a traditional command-line utility, proving that constraints can generate architecture rather than merely limit it.
````
