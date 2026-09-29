Compiling directly to Wasm

We could lower BAML MIR to Wasm, then use a browser or Wasmtime to execute it.
Wasmtime is a native runtime independent of V8. This makes Wasm a possible
server backend too, not just a browser feature.

wasm-encoder can produce module bytes. Binaryen can restructure control flow and
optimize it. Neither knows BAML's semantics. We still implement operations,
calls, errors, and runtime interfaces. Compiling the existing interpreter to
Wasm is different: it still reads BAML instructions at runtime.

Wasmtime normally compiles Wasm through Cranelift. We can also compile before
deployment and load the result without compiling again. Portable .wasm files and
these precompiled artifacts are different things. Precompiled output depends on
the target machine, Wasmtime version, and configuration, and must come from
trusted compiler output.

One runtime design compiles BAML's Rust support code to Wasm and keeps objects
in linear memory, an addressable region of bytes. Another leaves services in
native Rust and exposes calls through imports. Wasm addresses cannot be treated
as native Rust pointers. Both designs need an agreed interface and correct GC
rooting.

Wasmtime's sandbox does not automatically isolate individual BAML tasks. Tasks
inside one BAML heap still share objects. Separate instances with separate
memories can isolate programs; deliberately sharing memory removes that
separation for those bytes. Host imports also determine what a program can
access. Memory limits and execution budgets need configuration.

This is worth exploring if portable delivery or isolated server execution
matters. Against direct native output, possible costs include host calls, memory
restrictions, and runtime size. We need measurements. Compare cold loading, warm
execution, allocation, and GC under the same isolation requirements. Browser
details are in ../../shared/browser-wasm/README.txt.

Sources checked 2026-09-29 UTC.
https://docs.wasmtime.dev/examples-pre-compiling-wasm.html
https://docs.rs/wasmtime/49.0.1/wasmtime/struct.Module.html
https://docs.rs/wasmtime/49.0.1/wasmtime/struct.Store.html
https://docs.rs/wasmtime/49.0.1/wasmtime/struct.SharedMemory.html
