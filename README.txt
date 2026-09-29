BAML, faster

The project goal in the team notes is faster BAML execution, competitive with
V8 on realistic workloads with telemetry on. Memory use, startup, throughput,
and artifact size also matter. Compilation time is a cost to measure; making
the BAML compiler faster is not the main stated goal.

Generating Rust is the proposed means. It is not the success criterion.

Start with confirmed.txt for the verified facts and evidence for each option.
The individual notes also contain proposed experiments and opinions; those
are not established results.

The proposal favors ahead-of-time compilation: compile before deployment and
keep the interpreter for development and dynamically compiled BAML. It puts
JIT outside the initial scope because of its implementation and maintenance
cost. execution/aot.txt and execution/jit/README.txt explain that distinction.

Output choices live beside one another:

rust/              Generate Rust, then use rustc.
c-cpp/             Generate C/C++, then use Clang or another compiler.
llvm/              Generate LLVM IR directly.
direct-wasm/       Generate Wasm directly.
cranelift/         Generate native code with Cranelift.
custom-native/     Own the machine-code generator.

Inside rust/, compare MIR-to-Rust with checked-BAML-to-Rust. The team favors
Rust because of its existing runtime, but the notes do not establish a measured
winner. LLVM and C have recorded objections worth testing, not a proof they
cannot work. JIT and a custom native backend have a higher initial burden.

Native and browser execution are output targets, not separate decisions about
the language we generate. Rust, Clang, and LLVM can all lead to Wasm. That does
not automatically port BAML's I/O, scheduler, or shared moving heap to Workers.
Browser support needs its own execution tests. See
shared/browser-wasm/README.txt.

shared/ holds the runtime requirements, interpreter, team context, and evidence
record. next-step.txt proposes small comparisons before choosing a backend.
