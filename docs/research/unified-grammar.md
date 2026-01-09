# Unified Grammar

Idea: Create a unified grammar to represent knowledge in a generalized way that is easy to parse and manipulate to create a self-learning corpus of knowledge.

## Simple Logic as Grammar

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

## Beginning Conceptualization of a Generalized Logic System

The aim of the project is to create a logical system with rules and functions that is more general to human knowledge and conversation via a symbol AI approach. While Peano Arithmetic is a good proof-of-concept for proving fairly trivial theorems, we cannot directly extend this concept via more operators and axioms to represent the knowledge of more abstract mathematics let along social constructs and other branches of human knowledge.

A finite set of logical laws and axioms *cannot* represent language as per Gödel's incompleteness theorems. Natural language is also not strictly confinable to a logical system; any given system defined can be extended via a more intricate formalization. Therefore, bar some breakthrough allowing for self-growing formal systems, the platonic ideal of this project will be to create a complex *enough* logical system for some use cases.

That being said before we can approach a more generalized logic system, we need to first define the components of the previous P.A. implementation. As stated before, we can say that the core components are:

1. Truthfulness
    - Peano Arithmetic uses a classical, binary truth system (i.e. True or False).
2. Logical Operators (e.g. =, ∀, ∃)
    - Logical operators define relationships between expressions that exist at the truth-level. In PA, we have equality and quantifiers as our logical operators. It should be noted that they are not guaranteed to produce pure truth values as for instance in the statement `∀x (x + y = 0)`, the quantifier here can be approximately thought of as a function between Truth in a context dependent on a to-be-defined domain on x, y, to a Truth value dependent only on the to-be domain of x. This distinction is not strictly necessary for PA, but could be useful in more complex logical systems.
3. Domain Expressions (e.g. +, S, 0, x, y, z, ...)
    - Domain expressions represent terms and functions within the domain of discourse. In the case of PA, the domain of discourse is the natural numbers and operations on them. These expressions form the objects and morphisms in the category representing the domain.
    
With these concepts broken down, we can piece them back together differently to imagine what a simple observation-based knowledge system could look like. For a basic starting-point, consider a system which takes in observations and uses their information to resolve queries. If say the knowledge system is shown the following statements:

(Is (Apple) (Red))
(Is (Banana) (Yellow))
(Is (Apple) (Round))

And then queried with:

`(What (Apple))`

The system should be able to respond with:

`(Is (Apple) ( [ (Red, 0.5), (Round, 0.5) ] ))`

Since the system has been shown two observations about apples, it can respond with both properties with equal confidence. One could also imagine a system where these observations are themselves given confidence values, but for simplicity we will ignore that for now. For now we can attempt to define what exactly this system looks like in terms of our three components:

Truthfulness: There is no binary truth, we don't even necessarily have fuzzy confidence-based truth here, just observations. There is no concept of truthfulness in this system based solely on what is defined, it is more akin to a database of affiliations represented in a logical system. The important observation is that this system lacks truthfulness because it has no logical operators. In some sense this system is incomplete, however it is notable that we can still imagine a functional system without 2/3 of our components. 

Logical Operators: None.

Domain Expressions: The main functionality of this system comes from two intertwined domain expressions. 'Is' and 'What'. 'Is' represents an observation, a relationship between two terms. 'What' represents a query for all relationships affiliated with a term. These two expressions form the core of the system's functionality.

From this simple example, we have created a knowledge system with a clear gap in its structure. It lacks a lot of what seemingly makes a logical system useful despite still serving its own purpose. What exactly would adding logical operators and truthfulness look like in this system?

## Composite Logical System

TBD