# Corpus - A Monorepo for Logical Grammar and Rewriting Systems

This project explores unified grammars, axiom systems, and rewriting engines through a collection of reusable components and experimental tools.

## Project Structure

This is a Cargo workspace monorepo with the following high-level organization:

```
corpus/
├── Cargo.toml                 # Workspace root
├── .cargo/config.toml        # Workspace configuration
├── crates/                   # Core libraries and components
├── tools/                    # Experimental projects and utilities
└── docs/                     # Documentation
```

## Quick Start

### Building the Workspace
```bash
# Build all packages
cargo build --workspace
```

### Running the Peano Arithmetic Prover
```bash
# Run the prover CLI
cargo run --package peano-arithmetic --bin prover

# Example usage:
#   cargo run --bin prover -- "S(0) + 0 = S(0)"
#   cargo run --bin prover -- "0 + 0 = 0"
```

### Testing
```bash
# Test all packages
cargo test --workspace
```

## Overview of Components

### Core Crates

- **`corpus-core`**: Hash-consed node system and core data structures
- **`corpus-classical-logic`**: Classical logical operators (AND, OR, NOT, etc.)

### Tools

- **`peano-arithmetic`**: Theorem prover using priority queue search with Peano axioms

## Usage Examples

The Peano Arithmetic prover can prove statements like:
- `S(0) + 0 = S(0)` → Proved using axiom 3 (additive identity)
- `0 + 0 = 0` → Proved using axiom 3
- `S(0) + S(0) = S(S(0))` → Uses axiom 4 (additive successor)

For detailed documentation on each crate's purpose and API, see [CRATES.md](docs/CRATES.md).

For the type system hierarchy in category theory formulation, see [TYPE_SYSTEM.md](docs/TYPE_SYSTEM.md).

For the unified grammar specification, see [unified-grammar.md](docs/unified-grammar.md).

## Architecture

The project follows a layered architecture:

1. **Core Layer** (`corpus-core`): Hash-consed AST nodes, expression types
2. **Logic Layer** (`corpus-classical-logic`): Logical operator definitions
4. **Rewriting Layer** (`corpus-rewriting`): Rule-based transformation system
5. **Application Layer** (`peano-arithmetic`): Concrete theorem prover implementation

This modular design allows components to be reused across different logical systems and applications.
