# Examples

This document provides examples of using the Corpus framework.

## Peano Arithmetic Theorem Prover

### Basic Examples

#### Additive Identity

Prove that adding zero doesn't change a number:

```bash
cargo run --bin prover -- "S(0) + 0 = S(0)"
```

**Output:**
```
Theorem proven!

Proof path:
  1. S(0) + 0 = S(0) → S(0) = S(0)  [axiom3_additive_identity]
  2. S(0) = S(0) → true  [axiom1_reflexivity]
  → True  [Goal reached]
```

#### Multiple Successors

Prove addition with successor function:

```bash
cargo run --bin prover -- "S(0) + S(0) = S(S(0))"
```

**Proof Steps:**
1. Apply additive successor (Axiom 4): `S(0) + S(0) → S(S(0) + 0)`
2. Apply successor equality (Axiom 6): `S(S(0) + 0) → S(0) + 0`
3. Apply additive identity (Axiom 3): `S(0) + 0 → S(0)`
4. Apply reflexivity (Axiom 1): `S(0) = S(0) → true`

### Complex Examples

#### Deep Nesting

```bash
cargo run --bin prover -- "S(S(S(0))) + 0 = S(S(S(0)))"
```

**Analysis:**
- Requires 3 successor applications
- Shows how system handles nested structures
- Still uses simple axioms in sequence

#### Multiple Successor Additions

```bash
cargo run --bin prover -- "S(0) + S(S(0)) = S(S(S(0)))"
```

**Strategy:**
1. Apply axiom 4 multiple times to reduce successor depth
2. Apply axiom 6 to remove outer successors
3. Apply axiom 3 to eliminate zero
4. Apply axiom 1 to reach goal

## Understanding Proof Strategies

### The Prover's Approach

The theorem prover uses **A* search** with a **size-based heuristic**:

1. **Priority Queue**: Always explores smallest expressions first
2. **Bidirectional Rules**: Applies rules in both directions
3. **Top-Level + Inner**: Rewrites at all levels of the AST
4. **Goal-Checking**: Stops when both sides of equality are identical

### Why This Works

- **Simplification**: Rewriting often reduces expression size
- **Termination**: Bounded search prevents infinite loops
- **Completeness**: Explores all rewrites within bounds

### When Proofs Fail

A proof may fail if:
- **Theorem is false**: No valid transformation path exists
- **Search space too large**: `max_nodes` limit reached
- **Axioms insufficient**: Need more rewrite rules

## Debugging Failed Proofs

### Enable Trace Logging

```bash
RUST_LOG=corpus_core::proving=trace cargo run --bin prover -- "theorem"
```

This shows every rule application attempt, helping you understand why the prover couldn't find a proof.

### Increase Search Limit

Modify `max_nodes` in the prover initialization:

```rust
let mut prover: Prover<...> = Prover::new(100000, SizeCostEstimator, goal_checker);
```

## Using the Library

The Peano Arithmetic prover is built on reusable components. Here's how to use them:

### Define Your Own Logical System

```rust
use corpus_core::proving::{Prover, SizeCostEstimator};

// 1. Define your expression type
enum MyExpression {
    // ... custom expression variants
}

// 2. Implement required traits
impl Rewritable for MyExpression {
    // ... pattern matching and rewriting
}

impl Display for MyExpression {
    // ... formatting
}

// 3. Create prover
let prover = Prover::new(10000, SizeCostEstimator, MyGoalChecker);

// 4. Add rewrite rules
prover.add_rules(my_axioms);

// 5. Prove!
match prover.prove(&storage, initial_expr) {
    Some(result) => println!("Proof found!"),
    None => println!("No proof found"),
}
```

## Performance Notes

### Typical Performance

- **Simple theorems**: < 100 nodes explored, < 0.1s
- **Medium theorems**: 100-1000 nodes, < 1s
- **Complex theorems**: 1000-10000 nodes, < 10s

### Factors Affecting Performance

1. **Nesting depth**: More nesting = larger search space
2. **Successor applications**: Each S adds complexity
3. **Rule applicability**: More matching rules = more branching

### Optimization Tips

- Use **debug builds** during development
- Use **release builds** for production/performance testing
- Enable **trace logging** only when debugging
- Adjust **max_nodes** based on theorem complexity

## See Also

- [Getting Started](getting-started.md): Setup and basic usage
- [Logging](logging.md): Debugging and tracing
- [Research: Unified Grammar](../research/unified-grammar.md): Theoretical foundations
- [Infrastructure: Crates](../infrastructure/crates.md): API documentation
