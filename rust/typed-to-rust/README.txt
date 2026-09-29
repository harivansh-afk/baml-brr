Typed BAML to Rust

We would generate Rust before the compiler flattens functions into blocks and
jumps. A BAML loop would still be recognizable as a loop. The input must already
be type-checked and have names and calls resolved; copying source text is not
enough.

Skipping MIR does not inherently speed up the generated program. It could
save some compilation work or move that work elsewhere. Both outputs still
go through rustc, so execution speed depends on what code we give it.

BAML already records useful facts: expression types, resolved methods, matched
call arguments, and required conversions. A Rust emitter could reuse those. This
is the appealing part of Avery's source-oriented proposal: keep the program's
shape rather than reconstruct it later.

The cost is branching before existing MIR lowering has finished important work.
I checked one concrete example: a break or continue runs pending deferred
cleanup before jumping. If an exception occurs instead, cleanup runs through
separate handler blocks. A second emitter must share or reproduce both paths
correctly. Similar issues arise with closures, short-circuit expressions, and
method calls.

This still needs BAML's garbage collector and shared mutation rules. A BAML
object does not become an independently owned Rust object just because the
generated syntax looks like Rust. Waiting for I/O also needs a resumable
execution design.

Rust can produce Wasm, but that does not provide browser workers and shared-heap
scheduling automatically. The ordinary wasm32-unknown-unknown implementation of
thread spawning panics.

The question is whether preserving structure saves more work than repeating
semantic lowering. Translate a loop containing a captured variable, deferred
cleanup, and a caught error through both approaches. Compare how much compiler
logic each needs to duplicate before comparing speed.

Checked 2026-09-29 UTC.

../../../baml/baml_language/crates/baml_compiler2_hir_ty/src/infer.rs
../../../baml/baml_language/crates/baml_compiler2_mir/src/lower.rs
../../../call_notes.md
https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html
