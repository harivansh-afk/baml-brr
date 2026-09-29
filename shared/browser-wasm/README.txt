Running in the browser

Wasm is a portable instruction format. Chrome uses V8 to execute it; other
browsers have their own engines. Compiling BAML to Wasm can remove BEX's
interpreter loop, but it does not bypass the browser engine. A native BAML
executable can avoid V8 entirely. Those are different goals.

We could compile the existing interpreter to Wasm, generate Rust and compile
that to Wasm, or emit Wasm directly from BAML MIR. The first retains
interpretation. The other two compile the user's operations. See
../../options/wasm/README.txt for direct emission and native Wasmtime execution.

Running work in parallel needs Workers, which are browser threads, plus shared
Wasm memory and atomic operations for synchronization. SharedArrayBuffer exposes
shared bytes; it does not supply BAML's allocator, scheduler, or collector. The
site also needs cross-origin isolation, a browser security policy enabled
through deployment headers.

The current BAML code assumes single-threaded Wasm execution in EarlyYieldCheck.
Its request to pause threads for GC is compiled out on wasm32, and some engine
work uses spawn_local. Supporting several Workers over one moving heap therefore
needs explicit coordination: every Worker must stop while objects move, and
every live reference must be repaired. We have not demonstrated that working.

Rust's ordinary std::thread::spawn does not work on wasm32-unknown-unknown.
wasm-bindgen-rayon supplies Worker integration but currently requires nightly
and a rebuilt standard library. Stable generated Rust alone does not solve
browser threading.

I would first run one allocating BAML program in one Worker and verify waiting,
errors, and GC. Shared-heap parallelism comes after that. Neither proves a speed
advantage over JavaScript.

Sources checked 2026-09-29 UTC. Local:
../../../baml/baml_language/crates/bex_vm_types/src/lib.rs, EarlyYieldCheck;
../../../baml/baml_language/crates/bex_engine/src/lib.rs.
https://v8.dev/docs/wasm-compilation-pipeline
https://web.dev/articles/webassembly-threads
https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html
https://github.com/RReverser/wasm-bindgen-rayon
