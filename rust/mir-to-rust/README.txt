BAML MIR to Rust

We would take BAML after the compiler has turned each function into small blocks
of operations connected by jumps. Generate Rust for those blocks, then compile
it. Keep the existing runtime for objects, garbage collection, and I/O.

The attraction is reuse. MIR already expresses decisions that another backend
would otherwise have to repeat. The awkward part is turning its jumps back into
Rust loops and branches.

There is a simple fallback: keep a number saying which block runs next, then use
a Rust loop and match to execute that block. This removes individual bytecode
instruction dispatch, but still dispatches between blocks. Whether the optimizer
removes that remaining work needs measurement. We can later recover ordinary
loops and use the dispatcher only for difficult regions.

LLVM already does something similar for Wasm loops with multiple entrances. Its
pass inserts a dispatcher to give the loop one entrance. It produces another
graph, so borrowing it does not give us a finished Rust emitter.

We compiled a small two-entry loop using both Rust custom MIR and a dispatcher.
Both matched an arithmetic answer for 2,000 inputs. This establishes that the
fallback works for that example; it says nothing about BAML performance.

I would start here. The unresolved question is whether block dispatch and
runtime calls leave enough performance on the table to justify a more
complicated emitter. Test one ordinary loop, one difficult graph, and an object
surviving garbage collection. The control-flow note explains why we cannot yet
say BAML actually produces irreducible loops.

Checked 2026-09-29 UTC.

control-flow/README.txt
rust-custom-mir/probe.rs
../../../baml/baml_language/crates/baml_compiler2_mir/src/ir.rs
https://llvm.org/doxygen/WebAssemblyFixIrreducibleControlFlow_8cpp.html
