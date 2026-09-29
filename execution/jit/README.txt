Compiling while the program runs

JIT means compiling after the application starts. It does not necessarily mean
speculative optimization or rebuilding V8. AOT means compiling before
deployment. LLVM and Cranelift can support either timing.

A baseline JIT translates instructions into machine code with little
optimization, mainly removing interpreter dispatch: repeatedly deciding which
instruction handler to run. V8's Sparkplug is an example. An optimizing JIT
spends more time improving the generated code. A tracing JIT records frequently
executed paths and compiles those, with exits when execution takes another path.

Copy-and-patch is another implementation technique: assemble precompiled
machine-code templates and fill in their operands and destinations. CPython uses
it for optimized traces. It avoids needing LLVM during program execution, but
still needs template generation and runtime integration.

The notes reject JIT because of scope and maintenance. That is a reasonable
project decision. Mutation and concurrency do not make JIT impossible; LLVM ORC
explicitly supports concurrent execution and compilation. A conservative BAML
compiler could use known types without speculative guesses or a mechanism for
undoing those guesses.

The remaining work is substantial: calling conventions, executable memory,
errors, suspension, cancellation, and GC. If generated code holds an object
reference while the collector moves that object, the reference must be found and
updated. A backend library does not solve that for us.

I would keep JIT behind AOT unless executing dynamically supplied programs
quickly is central. Start with one integer function, then one allocating
function that survives GC. Measure compilation and execution separately. In
browsers, arbitrary native code generation is unavailable; we would need
interpretation or newly generated Wasm modules instead.

Sources checked 2026-09-29 UTC.
https://v8.dev/blog/sparkplug
https://llvm.org/docs/ORCv2.html
https://peps.python.org/pep-0744/
https://docs.rs/cranelift-jit/0.136.1/cranelift_jit/
