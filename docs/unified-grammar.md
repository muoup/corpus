# Unified Grammar

Idea: Create a unified grammar to represent knowledge in a generalized way that is easy to parse and manipulate to create a self-learning corpus of knowledge.

## Level 1: Simple Logic as Grammar

The first step to generalizing a logical system and grammar is to ensure that the framework we use can represent already established logical systems. To establish this, as a proof of concept found in tools/peano-arithmetic, we can represent Peano Arithmetic and proving simple theorems within it. 

As a quick refresher, Peano Arithmetic is a formal system that defines the natural numbers, addition, multiplication, and a successor function (i.e. f(x) = x + 1). We will ignore multiplication for now as it is simply an extension of the axiom system in a way that provides no novelty to the proof-of-concept. In a short and simple form, Peano Arithmetic works as such:

1. We have the grammar:

`<theorem> := <quantifiers> <relation>`

`<quantifiers> := <quantifiers> <quantifier> | ε` 
`<quanbtifier> := ∀ <variable> | ∃ <variable>`

`<relation> := <expression> = <expression> | <expression> < <expression>`

`<expression> := <term> | <term> + <expression>`

`<term> := 0 | S(<variable>) | <variable>`

Sample valid theorems in this grammar include:
- S(0) + S(0) = S(S(0))
- ∀x ∀y S(x) + S(S(y)) = S(S(x + S(y)))

2. We have axioms:

- A1: ∀x (x = x) -- Reflexivity of equality.
- A2: ∀x ¬(S(x) = 0) -- There is no natural number whose successor is 0.
- A3: ∀x ∀y (S(x) = S(y)) → (x = y) -- If the successors of two natural numbers are equal, then the numbers themselves are equal.
- A4: ∀x (x + 0 = x) -- Zero is the identity element over addition. 
- A5: ∀x ∀y (x + S(y) = S(x + y)) -- The successor of y added to x is the same as the successor of the sum of x and y.

While P.A. is defined via more axioms, some of these axioms, such as the existence of 0 and that S is closed over the natural numbers is too trivial to abstract in a programming implementation, we instead use these axioms to shape the AST and the implementation itself. It is in a sense the axioms for proving the soundness of the implementation. Additionally the system can be expanded upon in non-trivial ways, we are as mentioned ignoring multiplication and other logical symbols such as logical-or `||` and logical-and `&&` could be added for implementation-depth, but this limited set of axioms and grammar are sufficient to prove a plentiful set of theorems to discover and demonstrate the intricacies of building up the foundation of this codebase.

It is important to note that this project is not meant to replicate the proof-solving capabilities of already existing theorem provers such as Coq or Lean, but rather to demonstrate the knowledge capacities of traceable and consistent systems based on a rewrite-rule engine. Therefore, the means by which the demo for Peano Arithmetic solves these equations is to reimagine the above stated axioms as rewrite rules and to discover a path to a solution state by applying these rules.

For instance, the axioms can be converted into rules of manipulating a theorem AST:

For all expressions x, y
- R1 + R2: ? (See next section).
- R3: `S(x) = S(y)` can be rewritten as `x = y`
- R4: `x + 0` can be rewritten as `x`
- R5: `x + S(y)` can be rewritten as `S(x + y)`

As mentioned before, we also need to define the concept of a 'solution state', what the theorem prover is trying to reach to deduce the truthfulness of a theorem. Here we run into the question mark in R1 + R2. While separating the concept of a solution state is useful for implementation sake, these can be thought of as rewrite rules as well in and of itself.

- R1: `x = x` can be "rewritten" as `True`
- R2: `S(x) = x` can be "rewritten" as `False`

In practice however, these rules are implemented into the GoalChecker, which checks if the current AST matches either of these states, and halts search if so.

With the concept of rewrite rules and goal states established, we can actually solve simple theorems such as `S(0) + S(0) = S(S(0))` by applying the rules in a search algorithm until we reach a goal state. The search algorithm currently uses an A* style system with a size-based heuristic, figuring simpler theorems are more likely to be closer to a solution state. And with all of these points implemented, we can prove simple theorems in Peano Arithmetic via a rather robust, reliable, and reusable framework!