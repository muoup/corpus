# Type System Hierarchy for Defining Logical Systems

## Category Theory Formulation

This document describes the type system hierarchy for defining logical systems in approximately equivalent category theory formulation, to allow for planning future features through theoretical means.

---

## 1. The Foundational Category: **Hash-Consed Terms (H)**

### Objects
The objects of **H** are types `T` implementing `HashNodeInner`:

```rust
pub trait HashNodeInner: Sized {
    fn hash(&self) -> u64;        // Structural hash (morphism to ℕ)
    fn size(&self) -> u64;        // Size measure (morphism to ℕ)
    fn decompose(&self) -> Option<(u8, Vec<HashNode<Self>>)>;
    fn rewrite_any_subterm<F>(...) -> Option<HashNode<Self>>;
}
```

**Category Theory Interpretation:**
- Each `T: HashNodeInner` is an object representing a term algebra
- `hash(): T → u64` is a homomorphism from the term algebra to the monoid (u64, ⊕, 0)
- `size(): T → u64` is a homomorphism measuring structural complexity
- `decompose()` provides the coalgebraic structure for term deconstruction

### Morphisms
`HashNode<T>` represents hash-consed terms with canonical representation:

```rust
pub struct HashNode<T: HashNodeInner> {
    pub value: Rc<T>,
}
```

**Category Theory Interpretation:**
- `HashNode<T>` is a quotient of the free term algebra by the equivalence relation `t₁ ~ t₂ ⇔ hash(t₁) = hash(t₂)`
- This is the **coequalizer** of the hash function, ensuring structural sharing
- `Rc<T>` provides the memory coinduction for maximal sharing

### The Storage Monad: **NodeStorage**

```rust
pub struct NodeStorage<T: HashNodeInner> {
    nodes: RwLock<HashMap<u64, HashNode<T>, IdentityHasher>>,
}
```

**Category Theory Interpretation:**
- `NodeStorage<T>` is a **monad** `(T, return, bind)`
- `return: T → NodeStorage<T>` is `from_store: T → HashNode<T>`
- `bind` (implicit) chains hash-consing operations
- The monad laws ensure idempotency: `from_store(from_store(t)) = from_store(t)`

---

## 2. The Pattern Matching Category: **Pat**

### The Unifiable Trait

```rust
pub trait Unifiable: HashNodeInner + Clone {
    fn unify(
        pattern: &Pattern<Self>,
        term: &HashNode<Self>,
        subst: &Substitution<Self>,
        store: &NodeStorage<Self>
    ) -> Result<Substitution<Self>, UnificationError>;

    fn occurs_check(var_index: u32, term: &HashNode<Self>, subst: &Substitution<Self>) -> bool;
}
```

**Category Theory Interpretation:**
- `Unifiable` types form a category where morphisms are **unification problems**
- `unify()` is a partial binary operation `Pattern × Term ⇀ Substitution`
- This is the **pushout** in the category of substitutions
- `occurs_check` ensures the solution is a **pullback** (not infinite)

### The Pattern Functor

```rust
pub enum Pattern<T: HashNodeInner + Clone> {
    Variable(u32),              // Binding site (existential quantifier)
    Wildcard,                   // Anonymous variable
    Constant(T),                // Ground term
    Compound { opcode: u8, args: Vec<Pattern<T>> },  // Nested pattern
}
```

**Category Theory Interpretation:**
- `Pattern<T>` is a **functor** `F: H → H`
- `Variable` represents the **coproduct** injection `1 → T` (de Bruijn index)
- `Compound` represents the **product** `Opcode × Vector`
- Pattern matching is the **catamorphism** over this functor

### The Substitution Monoid

```rust
pub struct Substitution<T: HashNodeInner> {
    bindings: HashMap<u32, HashNode<T>>,
}
```

**Category Theory Interpretation:**
- Substitutions form a **monoid** `(Subst, ∘, id)`
- Composition: `σ ∘ τ` applies σ then τ
- Identity: `id` maps each variable to itself
- This is the **Kleisli category** of the substitution monad

---

## 3. The Rewrite System: **Rew**

### Rewrite Rules as Morphisms

```rust
pub struct RewriteRule<T: HashNodeInner + Unifiable, M: OpcodeMapper<T>> {
    pub name: String,
    pub pattern: Pattern<T>,
    pub replacement: Pattern<T>,
    pub direction: RewriteDirection,
    mapper: M,
}
```

**Category Theory Interpretation:**
- A rewrite rule is a **morphism** `pattern → replacement`
- Bidirectional rules are **isomorphisms**
- Forward-only rules are **epimorphisms** (surjective onto their image)
- The collection of rules forms a **rewrite category**

### Rewrite Direction

```rust
pub enum RewriteDirection {
    Both,       // Isomorphism: pattern ↔ replacement
    Forward,    // Epimorphism: pattern → replacement
    Backward,   // Monomorphism: replacement ← pattern
}
```

**Category Theory Interpretation:**
- `Both`: Equivalence relation generating a **congruence**
- `Forward`: Oriented rewrite relation (term rewriting system)
- `Backward`: Reverse oriented rewrite (anti-term rewriting)

### The Rewrite Functor

```rust
fn apply_substitution_to_pattern<T: HashNodeInner, M: OpcodeMapper<T>>(
    pattern: &Pattern<T>,
    subst: &Substitution<T>,
    store: &NodeStorage<T>,
    mapper: &M,
) -> HashNode<T>
```

**Category Theory Interpretation:**
- This is a **natural transformation** between substitution functors
- `mapper` is the **interpretation functor** from syntax to semantics
- The naturality square commutes due to the functor laws

---

## 4. The Prover Category: **Prov**

### The Prover as a Parametrized Monad

```rust
pub struct Prover<T: HashNodeInner + Clone, M: OpcodeMapper<T> + Clone,
                 C: CostEstimator<T>, G: GoalChecker<T>> {
    rules: Vec<RewriteRule<T, M>>,
    store: NodeStorage<T>,
    max_nodes: usize,
    cost_estimator: C,
    goal_checker: G,
}
```

**Category Theory Interpretation:**
- `Prover<T,M,C,G>` is a **parametrized monad** `P: H → H`
- Parameters: `(T, M, C, G)` form a product category
- The prover computes the **reflexive-transitive closure** of rewrite rules
- This is the **Kleene star** operation on the rewrite category

### The Search Strategy: A* as Initial Algebra

```rust
pub trait CostEstimator<T: HashNodeInner> {
    fn estimate_cost(&self, lhs: &HashNode<T>, rhs: &HashNode<T>) -> u64;
}

pub trait GoalChecker<T: HashNodeInner> {
    fn is_goal(&self, lhs: &HashNode<T>, rhs: &HashNode<T>) -> bool;
}
```

**Category Theory Interpretation:**
- `CostEstimator` is a **homomorphism** `H × H → (ℕ, ≤)` to a quantale
- The cost function is a **distance metric** satisfying triangle inequality
- `GoalChecker` is a **predicate** (morphism to the subobject classifier)
- A* search finds the **least fixed point** of the search functor

### Proof State as a Coalgebra

```rust
pub struct ProofState<T: HashNodeInner> {
    pub lhs: HashNode<T>,
    pub rhs: HashNode<T>,
    pub lhs_steps: Vec<ProofStep<T>>,
    pub rhs_steps: Vec<ProofStep<T>>,
    pub estimated_cost: u64,
}
```

**Category Theory Interpretation:**
- `ProofState<T>` is a **coalgebra** for the search functor
- `expand_state: State → P(State)` is the coalgebra structure map
- The prover finds the **final coalgebra** (terminal object) representing the proof

---

## 5. The Subterm Rewriting Adjunction

### The SubtermRewritable Trait

```rust
pub trait SubtermRewritable: Clone {
    type Expr: HashNodeInner;

    fn rewrite_any_subterm<F>(
        &self,
        store: &NodeStorage<Self::Expr>,
        try_rewrite: &F,
    ) -> Option<HashNode<Self::Expr>>
    where
        F: Fn(&HashNode<Self::Expr>) -> Option<HashNode<Self::Expr>>;
}
```

**Category Theory Interpretation:**
- This defines an **adjunction** `Subterm ⊣ TopLevel`
- Left adjoint: `Subterm` - descends into term structure
- Right adjoint: `TopLevel` - reassembles from rewritten parts
- The unit `η: T → Subterm(TopLevel(T))` is identity
- The counit `ε: TopLevel(Subterm(T)) → T` is the `rewrite_any_subterm` operation

### Blanket Implementation

```rust
impl<T: HashNodeInner> SubtermRewritable for HashNode<T> {
    type Expr = T;

    fn rewrite_any_subterm<F>(...) -> Option<HashNode<T>> {
        self.value.rewrite_any_subterm(self, store, try_rewrite)
    }
}
```

**Category Theory Interpretation:**
- This is the **adjoint functor theorem** application
- The natural isomorphism:
  ```
  Hom(Subterm(A), B) ≅ Hom(A, TopLevel(B))
  ```
- Domain-specific implementations override the counit for custom behavior

---

## 6. The Expression Construction Functor

### Construction and Decomposition

The `HashNodeInner` trait now provides both deconstruction and construction:

```rust
pub trait HashNodeInner: Sized {
    fn decompose(&self) -> Option<(u8, Vec<HashNode<Self>>)>;  // Deconstruction

    /// Construct an expression from an opcode and children.
    ///
    /// Returns `None` if the opcode is not valid for this type or if this type
    /// does not support compound expressions.
    fn construct_from_parts(
        opcode: u8,
        children: Vec<HashNode<Self>>,
        store: &NodeStorage<Self>,
    ) -> Option<HashNode<Self>> {
        None  // Default: types without compound expressions
    }
}
```

**Category Theory Interpretation:**
- `decompose` is the **coalgebra** for term deconstruction
- `construct_from_parts` is the **algebra** for term construction
- Together they form an **isomorphism** for well-formed terms
- Arity checking is implicit in the pattern matching

### The Free Term Algebra

Given:
- A set of opcodes `O` (with arity function `arity: O → ℕ`)
- The signature functor `F_T(X) = Σ_{o∈O} X^arity(o)`

Then:
- The category of **T-algebras** has objects `(X, α: F_T(X) → X)`
- `HashNode<T>` with `HashNodeInner` is the **free T-algebra**
- Pattern matching is the **catamorphism** (fold) over this algebra
- `construct_from_parts` provides the **algebra operation**

---

## 7. Domain-Specific Logical Systems

### The DomainContent Trait

```rust
pub trait DomainContent<T: TruthValue>
where
    Self: HashNodeInner,
    Self::Operator: HashNodeInner,
{
    type Operator: LogicalOperator<T>;
}
```

**Category Theory Interpretation:**
- `DomainContent<T>` defines a **slice category** over logical systems
- Objects are pairs `(T, Op)` where `Op: LogicalOperator<T>`
- Morphisms preserve both truth values and logical structure
- This is a **comma category** `(TruthValue ↓ LogicalOperator)`

### The Domain Expression Functor

```rust
pub enum DomainExpression<T: TruthValue, D: DomainContent<T>> {
    Domain(HashNode<D>),
    Logical(HashNode<LogicalExpression<T, D, D::Operator>>),
}
```

**Category Theory Interpretation:**
- `DomainExpression<T, D>` is a **coproduct** `D + LogicalExpression`
- This is the **colimit** of the diagram:
  ```
  D → LogicalExpression ← T
  ```
- The injection distinguishes domain atoms from logical compounds

### The Logical Expression Constructor

```rust
pub enum LogicalExpression<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T>> {
    Atomic(HashNode<D>),
    Compound { operator: Op, operands: Vec<HashNode<Self>>, _phantom: PhantomData<T> },
}
```

**Category Theory Interpretation:**
- This is an **inductive type** defined as an initial algebra
- Base case: `Atomic: D → LogicalExpression`
- Inductive case: `Compound: Op × Vectorⁿ(LogicalExpression) → LogicalExpression`
- The catamorphism over this type is **evaluation**

---

## 8. Peano Arithmetic as a Logical System

### The Arithmetic Expression Algebra

```rust
pub enum ArithmeticExpression {
    Add(HashNode<ArithmeticExpression>, HashNode<ArithmeticExpression>),
    Successor(HashNode<ArithmeticExpression>),
    Number(u64),
    DeBruijn(u32),
}
```

**Category Theory Interpretation:**
- This is the **initial algebra** for the PA signature:
  - `Add` has arity 2
  - `Successor` has arity 1
  - `Number` has arity 0 (constants)
  - `DeBruijn` has arity 0 (variables)
- This forms a **term model** of Peano Arithmetic

### The HashNodeInner Implementation

```rust
impl HashNodeInner for ArithmeticExpression {
    fn hash(&self) -> u64 { /* structural hashing */ }
    fn decompose(&self) -> Option<(u8, Vec<HashNode<Self>>)> { /* coalgebra */ }
    fn rewrite_any_subterm<F>(...) -> Option<HashNode<Self>> { /* recursion */ }
}
```

**Category Theory Interpretation:**
- `hash` is the **canonical homomorphism** to (u64, ⊕)
- `decompose` is the **coalgebra** for term deconstruction
- `rewrite_any_subterm` is the **recursion scheme** (paramorphism)

---

## 9. Theoretical Foundations for Future Features

### 9.1 Adding Higher-Order Logic

**Category Theory Foundation:**
- Current system is **first-order**: terms and formulas are separate sorts
- To add higher-order logic, introduce **exponentials** (function types)
- This gives a **cartesian closed category (CCC)**
- Lambda abstraction and application become adjoints:
  ```
  Hom(A × B, C) ≅ Hom(A, C^B)
  ```

**Implementation:**
```rust
pub enum HigherOrderExpression {
    Lambda { var: u32, body: HashNode<HigherOrderExpression> },
    Apply { func: HashNode<HigherOrderExpression>, arg: HashNode<HigherOrderExpression> },
    // ... existing variants
}
```

### 9.2 Dependent Type Theory

**Category Theory Foundation:**
- Dependent types form a **locally cartesian closed category (LCCC)**
- Each context `Γ` is an object
- Dependent product `Π` and sum `Σ` are right and left adjoints to substitution
- Pullback fibrations represent type dependencies

**Implementation:**
```rust
pub enum DependentType<T: HashNodeInner> {
    Pi { var: u32, domain: HashNode<Type<T>>, codomain: HashNode<Type<T>> },
    Sigma { var: u32, domain: HashNode<Type<T>>, codomain: HashNode<Type<T>> },
    // ...
}
```

### 9.3 Modal Logics

**Category Theory Foundation:**
- Modal operators are **monads** (□) and **comonads** (◇)
- Kripke semantics: **functors** between possible worlds
- Adjunctions: `□ ⊣ ◇` for S5, weaker connections for other modal systems

**Implementation:**
```rust
pub trait ModalLogic<T: HashNodeInner> {
    fn box_modal(expr: HashNode<T>) -> HashNode<T>;
    fn diamond_modal(expr: HashNode<T>) -> HashNode<T>;
    // Monad laws for box, comonad laws for diamond
}
```

### 9.4 Linear Logic

**Category Theory Foundation:**
- Linear logic types form a **symmetric monoidal category**
- Multiplicative connectives: tensor product ⊗, par ⅋, linear implication ⊸
- Exponential connectives: of-course !, why-not ?
- These are adjoint quadruples:
  ```
  A ⊸ B ⊣ (!A) ⅋ B
  A ⊗ B ⊣ A ⊸ (!B)
  ```

**Implementation:**
```rust
pub enum LinearLogicExpression {
    Tensor(Box<Self>, Box<Self>),
    Par(Box<Self>, Box<Self>),
    LinearImplication(Box<Self>, Box<Self>),
    OfCourse(Box<Self>),
    WhyNot(Box<Self>),
    // ...
}
```

### 9.5 Proof-Relevant Semantics

**Category Theory Foundation:**
- Proofs as terms: **Curry-Howard correspondence**
- Propositions are types, proofs are programs
- This is a **bi-category** or **indexed category**
- Objects are propositions, morphisms are proofs

**Implementation:**
```rust
pub struct ProofRelevant<T: HashNodeInner> {
    proposition: HashNode<T>,
    proof_term: HashNode<T>,
}
```

### 9.6 Categorical Semantics for Substitutions

**Current State:**
- `Substitution<T>` is a monoid of variable bindings
- Composition is sequential application

**Generalization:**
- Substitutions form a **presheaf** (contravariant functor to Set)
- Each term context is an object
- Substitutions are morphisms between contexts
- This is the **syntactic category** of the logic

**Implementation:**
```rust
pub struct Context<T: HashNodeInner> {
    variables: Vec<HashNode<Type<T>>>,
}

pub struct Substitution<T: HashNodeInner> {
    source: Context<T>,
    target: Context<T>,
    bindings: HashMap<u32, HashNode<T>>,
}
```

---

## 10. Summary: The Category of Logical Systems

The type system forms a **2-category** `LogSys`:

### Objects (Level 0)
- Logical systems (Peano Arithmetic, Classical Logic, etc.)
- Each is a category of terms with rewrite rules as morphisms

### 1-Morphisms (Level 1)
- **Translations** between logical systems
- Functors preserving the structure (hashing, rewriting)
- Example: Embedding PA into ZFC

### 2-Morphisms (Level 2)
- **Transformations** between translations
- Natural transformations commuting with the functors
- Example: Optimizing a translation by algebraic simplification

### Key Theorems

1. **Initiality**: The free term algebra is initial in the category of T-algebras
2. **Adjointness**: Subterm rewriting is adjoint to top-level rewriting
3. **Monadicity**: NodeStorage is a monad with hash-consing as the algebra
4. **Coalgebraic Finality**: Proof search finds the terminal coalgebra

### Files Implementing This Structure

| File | Category Theory Concept |
|------|------------------------|
| `crates/core/src/base/nodes.rs` | Objects and morphisms in **H** |
| `crates/core/src/rewriting/*.rs` | The **Rew** category and **Pat** functor |
| `crates/core/src/proving/mod.rs` | The **Prov** monad and search coalgebras |
| `crates/core/src/base/expression.rs` | Domain expressions as colimits |
| `tools/peano-arithmetic/src/syntax.rs` | PA as initial algebra |
| `tools/peano-arithmetic/src/opcodes.rs` | Signature algebra interpretation |

---

## Usage Guide for Planning Features

When planning a new feature, identify:
1. Which category the feature operates in (**H**, **Pat**, **Rew**, or **Prov**)
2. Whether it adds objects, morphisms, functors, or natural transformations
3. What adjunctions or limits/colimits are involved
4. Whether new monads/comonads are needed

Example: To add dependent types, we extend **H** to form an LCCC, which requires:
- Pullbacks (for substitution in types)
- Π and Σ as adjoints to weakening
- A comprehension schema (for type formation)
