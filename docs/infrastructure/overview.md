# Infrastructure Overview

The Corpus project follows a layered architecture with clear separation of concerns:

## Architecture Layers

### 1. Core Layer (`corpus-core`)
Foundational data structures and traits used across the entire workspace.

**Key Components:**
- Hash-consed node system for efficient AST representation
- Generic traits for logical operations and truth values
- Storage mechanism with thread-safe interning
- Base expression types

### 2. Logic Layer (`corpus-classical-logic`)
Classical propositional and first-order logic operators.

**Key Components:**
- Classical logical operators (AND, OR, NOT, IMPLIES, IFF, FORALL, EXISTS)
- Binary truth semantics
- Integration with core trait system

### 3. Rewriting Layer
Pattern-based transformation system (conceptual - part of core).

**Key Components:**
- Rewrite rule definitions
- Pattern matching and unification
- Bidirectional rule application
- Recursive rewriting strategies

### 4. Application Layer (`peano-arithmetic`)
Concrete implementations demonstrating the framework.

**Key Components:**
- Theorem prover using priority queue search
- Peano arithmetic axioms as rewrite rules
- Parser for theorem syntax
- Proof visualization

## Dependency Graph

```
peano-arithmetic
    ├─→ corpus-classical-logic
    │       └─→ corpus-core
    └─→ corpus-core
            └─→ rewriting (embedded)
```

## Design Philosophy

1. **Modularity**: Each crate has a single, well-defined responsibility
2. **Composability**: Higher-level crates compose lower-level ones through trait bounds
3. **Type Safety**: Heavy use of generics and traits ensure correctness at compile time
4. **Zero-Cost Abstractions**: Generic traits allow code reuse without runtime overhead
5. **Hash Consing**: Automatic deduplication ensures structural sharing and O(1) equality

## Component Interactions

### Flow from Bottom to Top

1. **Storage**: `HashNode<T>` provides deduplicated AST nodes
2. **Expressions**: Logical expressions build on `HashNode<T>`
3. **Patterns**: Rewrite rules use patterns to match expressions
4. **Rewriting**: Rules transform expressions using pattern matching
5. **Proving**: Provers orchestrate rewriting to find proofs

### Data Flow Example (Peano Arithmetic)

```
Theorem Input → Parser → HashNode Storage → Rewrite Rules → Prover Search → Result
```

## Extensibility Points

The system is designed to be extended in several ways:

- **New Logical Systems**: Implement `LogicalOperatorSet` trait for custom operators
- **New Truth Semantics**: Implement `TruthValue` trait for different truth systems
- **New Rewrite Strategies**: Implement custom search algorithms
- **New Applications**: Create tools using the rewriting engine
