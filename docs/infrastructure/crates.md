# Crate Documentation

This document provides detailed information about each cargo crate in the Corpus workspace, their purposes, and key functionality.

## Core Crates

### `corpus-core`

**Purpose**: Provides the foundational data structures and traits used across the entire workspace.

**Key Components**:

- **`HashNode<T>`**: Hash-consed nodes for efficient deduplication of AST structures
- **`NodeStorage<T>`**: Thread-safe storage for hash-consed nodes with interning
- **`Expression`**: AST expression type with binary operators and equality predicates
- **`LogicalOperatorSet<T, O>`**: Generic system for defining logical operators
- **`TruthValue`**: Trait for defining truth value semantics

**Features**:
- Automatic deduplication through hash-consing
- O(1) structural equality checks
- Memory-efficient storage of shared subtrees

**Dependencies**: None (foundational crate)

---

### `corpus-classical-logic`

**Purpose**: Implements classical propositional and first-order logic operators.

**Key Components**:

- **`ClassicalOperator`**: Enum defining classical operators (AND, OR, NOT, IMPLIES, IFF, FORALL, EXISTS)
- **`ClassicalLogicalSystem<T>`**: Pre-configured logical operator set with all classical operators
- **`BinaryTruth`**: Two-valued truth semantics (true/false)

**Features**:
- Ready-to-use classical logical system
- Integrates with `corpus-core`'s `LogicalOperatorSet`
- Generic over truth value types

**Dependencies**: `corpus-core`

**Use Case**: When you need standard classical logic operators for logical expressions.

---

### `corpus-unification`

**Purpose**: Provides pattern matching and unification with De Bruijn indices for variable binding.

**Key Components**:

- **`Pattern<T>`**: Pattern matching with support for:
  - `Variable(idx)`: De Bruijn-indexed variables
  - `Wildcard`: Matches anything
  - `Constant(c)`: Matches specific values
  - `Compound { opcode, args }`: Matches compound structures
- **`Substitution<T>`**: Maps De Bruijn indices to concrete terms
- **`Unifiable` trait**: Defines unification algorithm with occurs-check
- **`UnificationError`**: Error types for unification failures

**Features**:
- Robust unification with occurs-check (prevents infinite cycles)
- Support for De Bruijn indices for proper variable scoping
- Generic over term types

**Dependencies**: `corpus-core`

**Use Case**: Essential for pattern matching in rewrite rules and theorem proving.

---

### `corpus-rewriting`

**Purpose**: Provides a pattern-based rewriting rules engine for transforming AST structures.

**Key Components**:

- **`RewriteRule<T>`**: Represents a transformation rule with:
  - `pattern`: Left-hand side pattern to match
  - `replacement`: Right-hand side pattern to generate
  - `direction`: Both, Forward, or Backward application
- **`RewriteDirection`**: Controls rule application direction
- **`RewriteResult<T>`**: Contains transformed term and substitution

**Key Methods**:
- `try_match()`: Match pattern against term (forward)
- `try_match_reverse()`: Match replacement against term (backward)
- `apply()`: Apply rule forward with closure-based compound construction
- `apply_reverse()`: Apply rule backward

**Features**:
- Bidirectional rewrite rules
- Closure-based compound term construction for flexibility
- Integration with `corpus-unification` for pattern matching

**Dependencies**: `corpus-core`, `corpus-unification`

**Use Case**: When you need to apply transformation rules to AST structures (e.g., algebraic simplification, theorem proving).

---

## Tools

### `peano-arithmetic`

**Purpose**: A priority queue-based theorem prover for Peano Arithmetic that demonstrates the rewriting and unification systems.

**Key Components**:

- **`ArithmeticExpression`**: AST for Peano arithmetic terms:
  - `Add(a, b)`: Addition operation
  - `Successor(x)`: Successor function S(x)
  - `Number(n)`: Natural numbers
  - `DeBruijn(n)`: Variable bindings
- **`Prover`**: Priority queue-based search engine
  - Uses min-priority queue biased toward smaller trees
  - Applies rewrite rules bidirectionally
  - Explores both top-level and inner term rewritings
  - Limits search to configurable node count (~10k default)
- **`axioms.rs`**: Encodes Peano axioms as rewrite rules:
  - Axiom 2: `(S(x) = S(y)) ↔ (x = y)` (successor injectivity)
  - Axiom 3: `(x + 0) ↔ x` (additive identity)
  - Axiom 4: `(x + S(y)) ↔ S(x + y)` (additive successor)
- **`parsing.rs`**: Parses theorems in S-expression syntax
  - `S(0) + 0 = S(0)` → `EQ (PLUS (S(0)) (0)) (S(0))`
- **`prover.rs`**: Main prover implementation
- **`rewrite.rs`**: Rule application and subterm rewriting utilities
- **`patterns.rs`**: Pattern-specific operations for arithmetic expressions

**CLI Usage**:
```bash
cargo run --bin prover -- "S(0) + 0 = S(0)"
```

**Algorithm**:
1. Parse theorem into LHS and RHS terms
2. Initialize priority queue with initial state
3. Pop highest-priority state (smallest tree size)
4. Apply all rewrite rules to both sides (top-level and inner terms)
5. Generate new states with transformed terms
6. If LHS == RHS, proof found!
7. Repeat until queue empty or limit reached

**Dependencies**: `corpus-core`, `corpus-rewriting`, `corpus-classical-logic`, `corpus-unification`

**Use Case**: Demonstrates a complete application built on the corpus framework; serves as a reference for building theorem provers for other logical systems.

---

## Dependency Graph

```
peano-arithmetic
    ├─→ corpus-rewriting
    │       ├─→ corpus-core
    │       └─→ corpus-unification
    │               └─→ corpus-core
    ├─→ corpus-classical-logic
    │       └─→ corpus-core
    └─→ corpus-unification (already shown)
```

## Design Principles

1. **Zero-Cost Abstractions**: Generic traits allow code reuse without runtime overhead
2. **Hash Consing**: Automatic deduplication ensures structural sharing and O(1) equality
3. **Modularity**: Each crate has a single, well-defined responsibility
4. **Composition**: Higher-level crates compose lower-level ones through trait bounds
5. **Type Safety**: Heavy use of generics and traits ensure correctness at compile time

## Future Extensions

Potential additions to the workspace:

- **`crates/theorem-proofing`**: More general theorem proving strategies beyond priority queue
- **`crates/ai-interface`**: Natural language processing for querying logical systems
- **`tools/query-interface`**: AI-powered query processing over proven theorems
- **`tools/symbolic-execution`**: Program analysis using rewriting rules
- **`examples/`: Demonstrations and experiments across different logical systems
