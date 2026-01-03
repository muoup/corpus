# Corpus - A Monorepo for Logical Grammar and Rewriting Systems

This project explores unified grammars, axiom systems, and rewriting engines through a collection of reusable components and experimental tools.

## Project Structure

This is a Cargo workspace monorepo with the following organization:

```
corpus/
├── Cargo.toml                 # Workspace root
├── .cargo/config.toml        # Workspace configuration
├── crates/                   # Core libraries and components
│   ├── core/                 # Core types, traits, HashNode system
│   └── parser/               # Grammar parsing logic
├── tools/                    # Experimental projects and utilities
│   └── axiom-generator/      # Statement generation tool
└── docs/                     # Documentation
```

## Components

### crates/core
Core data structures and utilities:
- `HashNode<T>`: Hash-consed nodes for efficient deduplication
- `NodeStore<T>`: Thread-safe storage for hash-consed nodes
- AST types: `Proposition`, `Expression`, `Term`
- Hashable trait and implementations

### crates/parser
Grammar parsing and tokenization:
- `Lexer`: Tokenization of logical expressions
- `Parser`: Recursive descent parser for the unified grammar
- Support for De Bruijn indices, logical operators, and arithmetic

## Tools

### tools/axiom-generator
Demonstrates the core functionality by parsing logical statements and testing deduplication. This tool shows how the HashNode system efficiently stores identical logical expressions.

## Development

### Building the Workspace
```bash
# Build all packages
cargo build --workspace

# Or use the alias
cargo build-all
```

### Running Tools
```bash
# Run the axiom generator
cargo run --package axiom-generator

# Or use the alias
cargo run-axiom
```

### Testing
```bash
# Test all packages
cargo test --workspace
```

## Usage Examples

The axiom generator demonstrates parsing of statements like:
- `FORALL ( EQ ( S(/0) ) ( 0 ) )` → ∀(S(/0) = 0)
- `EQ ( PLUS ( S(0) ) ( S(0) ) ) ( S(S(0)) )` → (S(0) + S(0)) = S(S(0))

## Future Development

This structure supports adding:
- `crates/rewriting/`: Core rewriting engine components
- `crates/ai-interface/`: Natural language processing for queries
- `tools/query-interface/`: AI-powered query processing
- `examples/`: Demonstrations and experiments
- `tests/`: Integration tests across components

## Workspace Configuration

The workspace uses Cargo's workspace features for:
- Shared dependency management
- Unified building and testing
- Custom aliases defined in `.cargo/config.toml`

This organization provides a scalable foundation for investigating logical systems, axiom generation, and rewriting engines while maintaining clean separation of concerns and code reusability.