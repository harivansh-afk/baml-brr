MIR to Rust — first experiment

Code: baml/
Branch: brr/mir-to-rust
Base: 5956bc4af24521852e6193b1dd5338671f38f51b (canary)

Status: generated Rust now links bex_runtime, shared with BEX. The current
tests cover code generation, existing VM behavior, and native GC root handling.

The first step is BAML source -> existing checker -> existing MIR -> generated
Rust. Every MIR block becomes an arm of a Rust loop/match dispatcher. Keep the
MIR dump beside the generated source so we can follow each operation.

The current subset covers scalar integer/bool computation, branches, loops,
and direct calls. Unsupported MIR produces an explicit error. No heap objects,
GC integration, suspension, or telemetry equivalence is claimed in this step.

Integer range checks and VmPanic payloads now come from the same runtime
operations as BEX. Native code still does not materialize BAML exception
objects or implement catch/defer/tracing. That remains application-level work.
Its locals use native i64/bool storage, so a future timing would combine
representation changes with removing bytecode dispatch.

The raw MIR test has a two-entry cycle and a reversed block layout with a
nonzero entry. Both use the production emitter. Source tests exercise nested
break/continue, early return, joins, short-circuit guards, and argument order.
The emitter also rejects reads not initialized on every reachable path.

Read artifacts/scalar-loop/input.baml, then mir.txt and generated.rs in that
same directory. The generated executable returns 40 for input 10.
Those artifacts are local and ignored by Git; the example command recreates them.

Read runtime/README.txt for the crate split and next production slice. The
native-root tests use the actual moving collector, but array operations have
not yet been emitted from BAML. Scoped root frames and a pause protocol come
before treating this as an application backend.

The compiler crate's README.txt contains commands and the latest validation:
baml/baml_language/crates/baml_compiler2_rust/README.txt

Shared runtime code and GC proof:
baml/baml_language/crates/bex_runtime/README.txt

Existing evidence is in control-flow/ and ../custom-mir/. The earlier
hand-written custom-MIR example is separate from this source-driven emitter.

Use git -C baml diff HEAD to review all code changes. Performance comparisons belong
in the baseline machinery once this subset and its limits are agreed.
