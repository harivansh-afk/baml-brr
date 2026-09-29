Generating Rust

The proposal favors Rust because BAML's runtime is already written in it.
Generated functions can use the same support code and let rustc optimize and
produce machine code. This reduces some integration work; it does not establish
that Rust output is the fastest or easiest option overall.

There are two starting points to compare.

mir-to-rust/ starts after BAML has lowered the program into blocks and jumps.
We reuse those semantic decisions and translate the resulting control flow.
Its control-flow and custom-MIR folders contain the current investigations.

typed-to-rust/ starts from checked source structure. Loops remain recognizable,
but the generator must share or reproduce behavior currently handled by MIR
lowering, including captures, defaults, and cleanup.

Both routes still go through Rust's compiler. Skipping BAML MIR does not
inherently improve execution speed or even total compilation time. We need
to compare the code produced, compiler work repeated, and measured results.

Rust can emit native code or Wasm with the appropriate target and compatible
dependencies. A browser build still needs browser host bindings, scheduling,
and correct garbage collection. Shared-heap parallelism is a separate step.

Sources:
../../bamlrust.md
../../call_notes.md
https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html
