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

### Logging

The project uses the `log` facade with `env_logger` for structured logging.

#### Usage

```bash
# Default level (info, warnings, errors)
cargo run --bin prover -- "theorem"

# Enable debug logging
RUST_LOG=debug cargo run --bin prover -- "theorem"

# Enable trace logging (very detailed, includes all rewrite steps)
RUST_LOG=trace cargo run --bin prover -- "theorem"

# Filter to specific module
RUST_LOG=corpus_core::proving=debug cargo run --bin prover -- "theorem"

# Multiple modules with different levels
RUST_LOG=corpus_core::proving=debug,corpus_core::rewriting=trace cargo run --bin prover -- "theorem"
```

#### Log Levels

- `error`: Critical errors that prevent execution
- `warn`: Non-critical issues and exhaustion conditions
- `info`: High-level progress (proof start, success, milestones)
- `debug`: Proof search progress, cost estimates, statistics
- `trace`: Individual rule applications, pattern matches, parse steps

#### Performance

In release builds, `debug` and `trace` levels are compiled out via `log`'s level filtering, ensuring zero runtime overhead.

#### File Logging

`env_logger` supports file logging via configuration. Example for future use:
```rust
env_logger::Builder::new()
    .target(env_logger::Target::Pipe(Box::new(File::create("app.log").unwrap())))
    .init();
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
