Cranelift

Cranelift is another compiler backend, designed around quick compilation with
useful optimization. We would translate BAML MIR into its low-level
instructions. It can write native object files for deployment or generate
callable machine code while the program runs. Using it does not require a
V8-style system that guesses types and repeatedly recompiles functions.

The important tradeoff is compiler visibility. Current Cranelift supports
inlining: replacing a function call with the function's body. But we must
provide that body in Cranelift's format and decide when to inline. It does not
automatically see inside a Rust runtime compiled with LLVM. Frequent helper
calls could therefore hide optimization opportunities. That is a reason to
measure, not assume it will be slow.

Its current frontend also supports moving collectors. We mark live references;
it can save them before collection points and reload them afterward. Stack maps
describe where those references are saved. BAML must still connect those maps to
its collector and update the references. This is useful existing machinery, not
completed BAML support.

Cranelift emits native code, including aarch64 for Spark. It does not currently
emit browser Wasm. Wasmtime uses it in the opposite direction: Wasm becomes
native code. A browser product would need another output path.

There is also rustc_codegen_cranelift: generate Rust normally, then change
Rust's backend. That avoids writing our own Cranelift translation. It targets
faster debug builds, currently requires nightly, and has experimental panic
unwinding. It does not remove Rust frontend work or the browser gap.

I would test Cranelift if compilation speed or dynamic compilation matters.
Start with a native loop and a live object across collection. Choose based on
measured behavior, not a presumed performance ranking against LLVM.

Sources checked 29 September 2026:
https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/codegen/src/inline.rs
https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/frontend/src/frontend.rs
https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/codegen/src/isa/mod.rs
https://github.com/rust-lang/rustc_codegen_cranelift/blob/master/Readme.md
