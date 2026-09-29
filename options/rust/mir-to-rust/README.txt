MIR to Rust — first experiment

Code: baml/
Branch: brr/mir-to-rust
Base: 5956bc4af24521852e6193b1dd5338671f38f51b (canary)

Status: prototype code is written. The first test build stopped at the
two-minute command limit while compiling dependencies; tests have not run yet.

The first step is BAML source -> existing checker -> existing MIR -> generated
Rust. Every MIR block becomes an arm of a Rust loop/match dispatcher. Keep the
MIR dump beside the generated source so we can follow each operation.

Start with scalar integer/bool computation, branches, loops, and plain direct
calls. Unsupported features must produce an explicit error. No heap objects,
GC integration, suspension, or telemetry equivalence is claimed in this step.

The prototype must check BAML's signed 63-bit integer range. Its small Rust
error result is an experimental interface, not BAML's catchable exception
objects. That difference must be resolved before application benchmarks.
Its locals use native i64/bool storage, so a future timing would combine
representation changes with removing bytecode dispatch.

First establish correct generated execution. Then inspect remaining block
control flow and compiler output. Do not move runtime crates until a supported
feature actually needs that extraction.

The compiler crate's README.txt contains commands and the latest validation:
baml/baml_language/crates/baml_compiler2_rust/README.txt

Existing evidence is in control-flow/ and ../custom-mir/. The earlier
hand-written custom-MIR example is separate from this source-driven emitter.

Use git -C baml diff to review code changes. Performance comparisons belong
in the baseline machinery once this subset and its limits are agreed.
