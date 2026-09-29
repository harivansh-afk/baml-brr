MIR to Rust — first experiment

Code: baml/
Branch: brr/mir-to-rust
Base: 5956bc4af24521852e6193b1dd5338671f38f51b (canary)

Status: the source-driven example compiles and runs. Eight tests pass on
Rust 1.98.0, including same-checkout BEX comparisons and raw MIR edge cases.

The first step is BAML source -> existing checker -> existing MIR -> generated
Rust. Every MIR block becomes an arm of a Rust loop/match dispatcher. Keep the
MIR dump beside the generated source so we can follow each operation.

The current subset covers scalar integer/bool computation, branches, loops,
and direct calls. Unsupported MIR produces an explicit error. No heap objects,
GC integration, suspension, or telemetry equivalence is claimed in this step.

The prototype must check BAML's signed 63-bit integer range. Its small Rust
error result is an experimental interface, not BAML's catchable exception
objects. That difference must be resolved before application benchmarks.
Its locals use native i64/bool storage, so a future timing would combine
representation changes with removing bytecode dispatch.

The raw MIR test has a two-entry cycle and a reversed block layout with a
nonzero entry. Both use the production emitter. Source tests exercise nested
break/continue, early return, joins, short-circuit guards, and argument order.
The emitter also rejects reads not initialized on every reachable path.

Read artifacts/scalar-loop/input.baml, then mir.txt and generated.rs in that
same directory. The generated executable returns 40 for input 10.
Those artifacts are local and ignored by Git; the example command recreates them.

Next inspect this translation together before extending the subset. Runtime
crate extraction and performance measurements remain separate next steps.

The compiler crate's README.txt contains commands and the latest validation:
baml/baml_language/crates/baml_compiler2_rust/README.txt

Existing evidence is in control-flow/ and ../custom-mir/. The earlier
hand-written custom-MIR example is separate from this source-driven emitter.

Use git -C baml diff HEAD to review all code changes. Performance comparisons belong
in the baseline machinery once this subset and its limits are agreed.
