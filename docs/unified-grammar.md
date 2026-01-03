# Unified Grammar

Idea: Create a unified grammar to represent knowledge in a generalized way that is easy to parse and manipulate to create a self-learning corpus of knowledge.

## Level 1: Simple Logic as Grammar

Before expanding upon the idea of a unified grammar, a good starting point is to create a grammar around already granually formulated logical systems. For now, let's just consider Peano Arithmetic (PA) as a starting point. Consider a very simple grammar and the axioms of PA encoded within it:

<statement> ::= <statement> <binary-operator> <statement>
              | <quantifier> <statement>
              | ( <statement> )
              | S(<statement>)
              | <number>
              | <variable>
<binary-operator> ::= + | = | ->
<number> ::= 0 | 1 | 2 | 3 | ...
<quantifier> ::= ∀ <variable> | ∃ <variable>
<variable> ::= x | y | z | ...

Axioms:

1. ∀x (S(x) ≠ 0)
2. ∀x ∀y (S(x) = S(y) -> x = y)
3. ∀x (x + 0 = x)
4. ∀x ∀y (x + S(y) = S(x + y))

This grammar allows us to express statements in Peano Arithmetic, and the axioms provide the foundational truths of the system. The question becomes, what does using this grammar and logical system in a programmatic way entail? If we want to create a self-learning corpus of knowledge, we need some way to query previously learned statements, prove new statements, and store them in a way that is easy to retrieve.

Ignoring parsing and syntax trees, as they are relatively straight-forward, the most obvious approach to handling proven knowledge is via a HashMap or some other key-value store, where all proven statements have some hashing scheme consistent over any sub-AST to allow for easy retrieval. This does however post a few challenges. Say for instance we have two statements:

1. ∀x ∀y (x + S(y) = S(x + y))
1. ∀x ∀y (S(x) + y = S(x + y))

These statements, while logically equivalent, are not syntactically equivalent, and even with some trivial normalization scheme, still poses a challenge to unify the hashing scheme. For now, the likely best simple approach is to implement some nornmalization rules and store any duplcate statements we can't quite squash. Consider for instance the following modified grammar:

# Proposition :: Boolean
<proposition> ::= <logical-binop> (<proposition>) (<proposition>) 
                | <logical-unop> (<proposition>) 
                | <quantifier> (<proposition>) 
                | <logical-expression>
<logical-binop> ::= ∧ | ∨ | ->
<logical-unop> ::= ¬
<quantifiers> ::= <quantifiers> <quantifier> | ε
<quantifier> ::= ∀ | ∃

# Logical Expression :: Boolean
<logical-expression> ::= = (<expression>) (<expression>)

# Expression :: Term
<expression> ::= <term> 
               | <binary-operator> (<expression>) (<expression>)
<term> ::= S(<term>) | <number> | <De Bruijn-index>
<number> ::= 0 | 1 | 2 | 3 | ...
<De Bruijn-index> ::= /0 | /1 | /2 | /3 | ...