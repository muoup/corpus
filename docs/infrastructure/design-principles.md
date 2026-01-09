# Design Principles

This document outlines the core design principles that guide the Corpus project architecture.

## 1. Zero-Cost Abstractions

**Principle:** Use generic traits to provide abstractions without runtime overhead.

**Application:**
- `HashNode<T>`: Generic over any node type, no boxing overhead
- `LogicalOperatorSet<T, O>`: Generic over both expression and operator types
- `TruthValue` trait: Compile-time polymorphism for truth semantics

**Benefit:** Code reuse across different logical systems with no performance penalty.

## 2. Hash Consing

**Principle:** Automatically deduplicate structurally equal nodes to enable O(1) equality and memory efficiency.

**Application:**
- `NodeStorage<T>`: Central storage with interning
- Shared subtrees: Common subexpressions stored once
- Structural equality: Single hash comparison instead of tree traversal

**Benefit:**
- O(1) equality checks instead of O(n) tree comparisons
- Reduced memory usage for shared subtrees
- Simplified caching and memoization

## 3. Modularity

**Principle:** Each crate has a single, well-defined responsibility.

**Application:**
- `corpus-core`: Foundation only - storage, nodes, traits
- `corpus-classical-logic`: Classical logic operators only
- `peano-arithmetic`: Demo application only

**Benefit:**
- Clear boundaries and interfaces
- Independent testing and evolution
- Easy to understand and maintain

## 4. Composition

**Principle:** Higher-level crates compose lower-level ones through trait bounds.

**Application:**
```rust
// peano-arithmetic composes:
// - corpus-core (HashNode, Storage)
// - corpus-classical-logic (ClassicalLogicalExpression)
// - rewriting (RewriteRule, Rewritable)
```

**Benefit:**
- Mix and match components as needed
- Substitute implementations without changing consuming code
- Progressive enhancement of functionality

## 5. Type Safety

**Principle:** Use Rust's type system to prevent errors at compile time.

**Application:**
- Phantom types for domain-specific storage
- Trait bounds to enforce valid combinations
- Enum exhaustiveness for operator handling

**Benefit:**
- Catch errors before runtime
- Self-documenting code
- IDE-friendly with autocomplete

## 6. Trait-Based Architecture

**Principle:** Define behaviors through traits, not concrete implementations.

**Application:**
- `Rewritable`: Can rewrite to pattern and back
- `TruthValue`: Can evaluate to truth values
- `LogicalOperatorSet`: Can provide operators and combine expressions

**Benefit:**
- Multiple implementations coexist
- Easy to add new types
- Test in isolation

## 7. Performance Consciousness

**Principle:** Design for performance from the start.

**Application:**
- No logging in hot paths (except aggregated stats)
- Copy-on-write patterns where appropriate
- Avoid allocations in tight loops
- Compile-time optimization flags

**Benefit:**
- Fast proof search
- Scalable to complex theorems
- Minimal overhead from abstractions

## Trade-offs

### Generality vs. Performance
- **Choice:** Generic traits over specialized code
- **Trade-off:** Slightly higher compile times, but zero runtime cost
- **Decision:** Favor generics for API surface, specialize in hot paths

### Simplicity vs. Expressiveness
- **Choice:** Simple rewriting engine vs. advanced strategies
- **Trade-off:** Easier to understand and maintain vs. more powerful search
- **Decision:** Start simple, add complexity as needed

### Completeness vs. Usability
- **Choice:** Limited operator set vs. comprehensive coverage
- **Trade-off:** Faster development vs. broader applicability
- **Decision:** Minimal viable operator set for proof of concept
