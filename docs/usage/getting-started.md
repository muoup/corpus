# Getting Started

Welcome to Corpus! This guide will help you get up and running quickly.

## Prerequisites

- **Rust**: Version 1.70 or later
- **Cargo**: Comes with Rust installation

## Installation

### Clone the Repository

```bash
git clone <repository-url>
cd corpus
```

### Build the Workspace

```bash
# Build all packages
cargo build --workspace

# Build with optimizations
cargo build --release --workspace
```

## Quick Demo: Peano Arithmetic Prover

The fastest way to see Corpus in action is to run the Peano Arithmetic theorem prover.

### Run a Simple Theorem

```bash
cargo run --bin prover -- "S(0) + 0 = S(0)"
```

Expected output:
```
Parsing theorem: S(0) + 0 = S(0)
Theorem: S(0) + 0 = S(0)

Searching for proof (max 10000 nodes)...
Theorem proven!

Proof path:
  1. S(0) + 0 = S(0) → S(0) = S(0)  [axiom3_additive_identity]
  2. S(0) = S(0) → true  [axiom1_reflexivity]
  → True  [Goal reached]
```

### Try Other Theorems

```bash
# Additive identity
cargo run --bin prover -- "0 + 0 = 0"

# Additive successor
cargo run --bin prover -- "S(0) + S(0) = S(S(0))"

# More complex
cargo run --bin prover -- "S(S(0)) + S(0) = S(S(S(0)))"
```

## Understanding the Output

### Proof Path

Each step shows:
1. **Input expression**: The expression before transformation
2. **Output expression**: The expression after transformation
3. **Rule applied**: Which axiom or rule was used

### Search Statistics

- **max nodes**: Maximum number of states explored (configurable)
- **nodes explored**: Actual number of states checked
- **Goal reached**: Whether a solution was found

## Next Steps

- **[Logging Guide](logging.md)**: Learn about debugging and tracing proof searches
- **[Examples](examples.md)**: More complex theorems and usage patterns
- **[Infrastructure Overview](../infrastructure/overview.md)**: Understand the system architecture
- **[Research](../research/)**: Explore the theoretical foundations

## Troubleshooting

### Build Failures

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build --workspace
```

### Theorem Prover Exhausted

If you see "No conclusion found (max search depth reached)":
- The theorem may be false
- The search space may be too large (try increasing `max_nodes`)
- The axioms may be insufficient

### Parse Errors

If you see "Parse error":
- Check theorem syntax (parentheses, keywords)
- Refer to [Peano Arithmetic](../research/unified-grammar.md) for valid grammar
- Use `--` to separate theorem from other arguments

## Development Setup

### Running Tests

```bash
# Test all packages
cargo test --workspace

# Test specific package
cargo test -p corpus-core

# Test with output
cargo test --workspace -- --nocapture
```

### IDE Support

The project works well with:
- **VS Code** + rust-analyzer extension
- **IntelliJ IDEA** + Rust plugin
- **CLion** + Rust plugin

Configure your IDE to use the workspace root (`/path/to/corpus`) as the project directory.
