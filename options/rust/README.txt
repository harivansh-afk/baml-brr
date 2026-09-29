Generate Rust

mir-to-rust/ translates BAML's lowered operations and control-flow graph.
checked-to-rust/ starts from checked source structure and must account for
work currently done during MIR lowering.
custom-mir/ explores Rust's internal, nightly-only MIR interface.

The first two generate ordinary Rust source. They share the same questions
about values, GC roots, errors, and suspension. Fewer compiler stages do not
establish faster execution.

Only mir-to-rust currently has a BAML worktree. The custom-MIR folder retains
the earlier small control-flow probe, not a BAML compiler implementation.

Rust can target native code or Wasm. Browser host operations and shared-heap
Workers remain separate runtime work.
