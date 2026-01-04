use corpus_core::nodes::{HashNode, HashNodeInner, NodeStorage};
use corpus_rewriting::RewriteRule;
use corpus_unification::Unifiable;

use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

#[derive(Clone)]
pub struct ProofStep<T: HashNodeInner> {
    pub rule_name: String,
    pub old_expr: HashNode<T>,
    pub new_expr: HashNode<T>,
}

pub struct ProofState<T: HashNodeInner + Unifiable> {
    pub lhs: HashNode<T>,
    pub rhs: HashNode<T>,
    pub lhs_steps: Vec<ProofStep<T>>,
    pub rhs_steps: Vec<ProofStep<T>>,
    pub estimated_cost: u64,
}

pub struct Prover<T: HashNodeInner + Unifiable> {
    rules: Vec<RewriteRule<T>>,
    store: NodeStorage<T>,
    max_nodes: usize,
}

impl<T: HashNodeInner + Unifiable> Prover<T> {
    pub fn new(max_nodes: usize) -> Self {
        Self {
            rules: Vec::new(),
            store: NodeStorage::new(),
            max_nodes,
        }
    }

    pub fn add_rule(&mut self, rule: RewriteRule<T>) {
        self.rules.push(rule);
    }

    pub fn prove(
        &self,
        initial_lhs: &HashNode<T>,
        initial_rhs: &HashNode<T>,
    ) -> Option<ProofResult<T>> {
        let mut heap = BinaryHeap::new();
        let mut visited: HashSet<(u64, u64)> = HashSet::new();
        let mut nodes_explored = 0usize;

        let initial_cost = self.estimate_cost(initial_lhs, initial_rhs);
        let initial_state = ProofState {
            lhs: initial_lhs.clone(),
            rhs: initial_rhs.clone(),
            lhs_steps: Vec::new(),
            rhs_steps: Vec::new(),
            estimated_cost: initial_cost,
        };

        heap.push(initial_state);

        while let Some(state) = heap.pop() {
            nodes_explored += 1;

            if nodes_explored > self.max_nodes {
                return None;
            }

            let lhs_hash = state.lhs.hash();
            let rhs_hash = state.rhs.hash();

            if lhs_hash == rhs_hash {
                return Some(ProofResult {
                    lhs_steps: state.lhs_steps,
                    rhs_steps: state.rhs_steps,
                    nodes_explored,
                    final_expr: state.lhs,
                });
            }

            let key = (lhs_hash, rhs_hash);
            if visited.contains(&key) {
                continue;
            }
            visited.insert(key);

            for successor in self.expand_state(&state) {
                heap.push(successor);
            }
        }

        None
    }

    fn expand_state(&self, state: &ProofState<T>) -> Vec<ProofState<T>> {
        let mut successors = Vec::new();

        for rule in &self.rules {
            if rule.is_bidirectional() {
                if let Ok(_subst) = rule.try_match(&state.lhs, &self.store) {
                    let new_lhs = state.lhs.clone();
                    let new_cost = self.estimate_cost(&new_lhs, &state.rhs);
                    let mut lhs_steps = state.lhs_steps.clone();
                    lhs_steps.push(ProofStep {
                        rule_name: rule.name.clone(),
                        old_expr: state.lhs.clone(),
                        new_expr: new_lhs.clone(),
                    });
                    successors.push(ProofState {
                        lhs: new_lhs,
                        rhs: state.rhs.clone(),
                        lhs_steps,
                        rhs_steps: state.rhs_steps.clone(),
                        estimated_cost: new_cost,
                    });
                }

                if let Ok(_subst) = rule.try_match(&state.rhs, &self.store) {
                    let new_rhs = state.rhs.clone();
                    let new_cost = self.estimate_cost(&state.lhs, &new_rhs);
                    let mut rhs_steps = state.rhs_steps.clone();
                    rhs_steps.push(ProofStep {
                        rule_name: rule.name.clone(),
                        old_expr: state.rhs.clone(),
                        new_expr: new_rhs.clone(),
                    });
                    successors.push(ProofState {
                        lhs: state.lhs.clone(),
                        rhs: new_rhs,
                        lhs_steps: state.lhs_steps.clone(),
                        rhs_steps,
                        estimated_cost: new_cost,
                    });
                }
            }
        }

        successors
    }

    fn estimate_cost(&self, lhs: &HashNode<T>, rhs: &HashNode<T>) -> u64 {
        let lhs_size = lhs.size();
        let rhs_size = rhs.size();
        let lhs_hash = lhs.hash();
        let rhs_hash = rhs.hash();

        let hash_distance = if lhs_hash > rhs_hash {
            lhs_hash - rhs_hash
        } else {
            rhs_hash - lhs_hash
        };

        lhs_size + rhs_size + hash_distance
    }
}

impl<T: HashNodeInner + Unifiable> PartialEq for ProofState<T> {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost == other.estimated_cost
    }
}

impl<T: HashNodeInner + Unifiable> Eq for ProofState<T> {}

impl<T: HashNodeInner + Unifiable> PartialOrd for ProofState<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: HashNodeInner + Unifiable> Ord for ProofState<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimated_cost.cmp(&self.estimated_cost)
    }
}

pub struct ProofResult<T: HashNodeInner> {
    pub lhs_steps: Vec<ProofStep<T>>,
    pub rhs_steps: Vec<ProofStep<T>>,
    pub nodes_explored: usize,
    pub final_expr: HashNode<T>,
}

impl<T: HashNodeInner + Unifiable> ProofResult<T> {
    pub fn print(&self)
    where
        T: std::fmt::Display,
    {
        println!("✓ Theorem proved!");
        println!("Nodes explored: {}", self.nodes_explored);
        println!();

        if !self.lhs_steps.is_empty() {
            println!("LHS transformations:");
            for (i, step) in self.lhs_steps.iter().enumerate() {
                println!("  {}. Apply \"{}\":", i + 1, step.rule_name);
                println!("     {} → {}", step.old_expr, step.new_expr);
            }
            println!();
        }

        if !self.rhs_steps.is_empty() {
            println!("RHS transformations:");
            for (i, step) in self.rhs_steps.iter().enumerate() {
                println!("  {}. Apply \"{}\":", i + 1, step.rule_name);
                println!("     {} → {}", step.old_expr, step.new_expr);
            }
            println!();
        }

        println!("Final: {} = {} ✓", self.lhs_steps.last().map_or(&self.final_expr, |s| &s.new_expr), self.rhs_steps.last().map_or(&self.final_expr, |s| &s.new_expr));
    }
}
