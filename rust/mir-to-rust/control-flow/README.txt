Does BAML actually have the difficult kind of control flow?

We do not know yet. A graph having jumps is ordinary. The difficult case is a
loop with multiple entrances: execution can enter different parts of the cycle
without passing through one common loop header. This is called irreducible
control flow.

Rust source and Wasm cannot directly spell every such graph, but they can run
equivalent code after a transformation. One solution is a variable saying which
block runs next and a loop that dispatches on that variable. LLVM uses this idea
when repairing Wasm control flow. This is a possible cost, not a reason Rust
output cannot work.

The existing snapshot audit examined 55 functions across 20 files and found no
irreducibility. Only two functions contained cycles. These were committed
snapshots, not freshly compiled programs. The audit included printed unwind
edges but omitted implicit exception edges and catch-region metadata. The result
is much too narrow to establish a guarantee.

The source gives a more useful next step. Ordinary while lowering builds a
condition, body, optional step, and exit. Break and continue replay deferred
cleanup before jumping. Exceptions take a different path through cleanup
handlers. Inspecting loop scaffolding alone therefore misses meaningful
execution paths.

The next check should inspect actual MIR before and after optimization for
nested loops, breaks, continues, catches, and deferred cleanup. Include implicit
error paths, not just the successors printed beside each block. Save a small
failing example if one appears.

Meanwhile, the custom-MIR probe demonstrates one deliberately irreducible
example and its working Rust dispatcher equivalent. It is not evidence about
which graphs BAML emits.

Checked 2026-09-29 UTC against BAML 5956bc4af245.

snapshot-audit.json
../../../../learning/audit_mir_snapshots.py
../../../../baml/baml_language/crates/baml_compiler2_mir/src/lower.rs
https://llvm.org/doxygen/WebAssemblyFixIrreducibleControlFlow_8cpp.html
