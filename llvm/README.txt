Direct LLVM output

LLVM is a compiler backend: it turns low-level operations into optimized machine
code. We would translate BAML MIR into LLVM's instruction format, then let LLVM
handle CPU instructions and registers. Rust normally uses LLVM too. Skipping
Rust does not automatically produce faster programs.

LLVM accepts blocks connected by arbitrary branches. It can represent loops with
multiple entrances, so we would not need to reconstruct nested Rust loops first.
That does not mean BAML currently produces such loops. LLVM can also emit Wasm
and already has a pass that repairs those graphs for Wasm's structured control
flow.

The main opportunity is control over what the optimizer sees. We can expose
arithmetic and object operations directly instead of hiding them behind runtime
calls. But more direct operations mean more BAML behavior we must implement
correctly. We can still reuse the Rust runtime for allocation, errors, and other
services.

Calls into that runtime need an agreement about arguments, layouts, and errors.
A C-compatible interface is practical, not compulsory for every generated call.
LLVM can optimize across compatible LLVM inputs, including Rust, when linking is
configured appropriately.

Moving garbage collection is the difficult part. LLVM has support for recording
live references and replacing them after objects move. It does not automatically
connect to BAML's collector. Its statepoint interfaces also make no
compatibility guarantee between versions.

I would defer this until Rust output exposes a specific limitation. The first
experiment should compile one integer loop, then keep one object alive across
forced collection. Compare correctness, compilation time, execution time, and
size. Arbitrary branches alone are not enough reason to own a separate compiler
integration.

Sources checked 29 September 2026:
https://llvm.org/docs/LangRef.html
https://llvm.org/docs/Statepoints.html
https://doc.rust-lang.org/rustc/linker-plugin-lto.html
https://github.com/llvm/llvm-project/blob/main/llvm/lib/Target/WebAssembly/WebAssemblyFixIrreducibleControlFlow.cpp
