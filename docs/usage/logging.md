# Logging Guide

The project uses the standard Rust `log` facade with `env_logger` for structured, configurable logging.

## Usage

### Basic Logging

```bash
# Default level (info, warnings, errors)
cargo run --bin prover -- "theorem"

# Enable debug logging
RUST_LOG=debug cargo run --bin prover -- "theorem"

# Enable trace logging (very detailed)
RUST_LOG=trace cargo run --bin prover -- "theorem"
```

### Module-Specific Logging

```bash
# Only trace the proving module
RUST_LOG=corpus_core::proving=trace cargo run --bin prover -- "theorem"

# Multiple modules with different levels
RUST_LOG=corpus_core::proving=debug,corpus_core::rewriting=trace cargo run --bin prover -- "theorem"

# Everything at info level, but specific module at trace
RUST_LOG=info,corpus_core::proving=trace cargo run --bin prover -- "theorem"
```

## Log Levels

### error
**When:** Critical errors that prevent execution
**Example:** Parse errors, missing dependencies
**Output:** `Parse error: ...`

### warn
**When:** Non-critical issues and exhaustion conditions
**Example:** Proof search exceeded max depth
**Output:** `Proof search exhausted: explored 10000 nodes`

### info
**When:** High-level progress and milestones
**Example:** Proof search started, axiom loaded, goal reached
**Output:**
```
Loaded 12 rewrite rules
Goal reached! Exploring expression: true
Proof found! Steps: 2, Nodes explored: 5
```

### debug
**When:** Proof search progress, cost estimates, statistics
**Example:** Initial cost estimation, periodic progress updates
**Output:**
```
Starting proof search with max_nodes=10000
Initial cost: 7
Explored 1000/10000 nodes
```

### trace
**When:** Individual rule applications, pattern matches, parse steps
**Example:** Each rule application attempt
**Output:**
```
Applying rule: axiom1_reflexivity
Applying rule: axiom3_additive_identity
Rule axiom3_additive_identity generated 1 rewrites for S(0) + 0 = S(0)
Skipping already-visited expression (hash: 1a2b3c4d)
```

## Performance Considerations

### Release Builds

In release builds, `debug` and `trace` logs are **compiled out** entirely via `log`'s level filtering. This ensures zero runtime overhead when detailed logging is not needed.

### Hot Paths

Logging is carefully placed to avoid performance impact:
- **No string formatting in tight loops** (rule application, pattern matching)
- **Periodic stats instead of per-iteration** (every 1000 nodes)
- **Conditional logging** only when relevant (rewrites found, hash skips)

### Enabling Hot Path Logs

You can safely enable `trace` logging in development for debugging, but be aware it may slow down the prover significantly because it logs every rule application attempt.

## Examples

### Debug a Slow Proof Search

```bash
# See which rules are being tried
RUST_LOG=corpus_core::proving=trace cargo run --bin prover -- "complex-theorem"
```

### Monitor Progress on Long Searches

```bash
# Get periodic updates
RUST_LOG=info cargo run --bin prover -- "long-theorem"
```

### Trace Parsing Issues

```bash
# See how theorem is being parsed
RUST_LOG=peano_arithmetic::parsing=trace cargo run --bin prover -- "theorem"
```

### Quiet Mode

```bash
# Only show user output, suppress all logs
RUST_LOG=error cargo run --bin prover -- "theorem"
```

## Adding Logging to Your Code

### In Library Code

```rust
use log::{debug, info, warn, error};

// Debug-level: Detailed internal state
debug!("Initial cost: {}", cost);

// Info-level: Milestones
info!("Loaded {} rewrite rules", rules.len());

// Warn-level: Expected issues
warn!("Proof search exhausted: explored {} nodes", count);

// Error-level: Critical failures
error!("Parse error: {}", err);
```

### In Application Code

```rust
use log::{info};

// Application-level logging
info!("Processing theorem: {}", theorem);
```

## Output Format

Log output uses timestamps in seconds:
```
[2026-01-09T19:17:35Z INFO  prover] Loaded 12 rewrite rules
```

## Future: File Logging

The current setup writes to stdout, but `env_logger` supports file logging:

```rust
// Future enhancement in tools/peano-arithmetic/src/bin/prover.rs
env_logger::Builder::new()
    .target(env_logger::Target::Pipe(Box::new(File::create("app.log").unwrap())))
    .init();
```

This would allow logging to a file while keeping console output clean for user-facing messages.
