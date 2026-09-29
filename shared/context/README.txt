Where the team seems to be

The notes agree on the broad idea: compile BAML ahead of time, reuse the
existing runtime, and compare real programs with V8. Ahead of time means the
compiler runs before the application starts. Keeping the interpreter for
development and dynamically supplied code is still useful.

They do not agree on where to start generating Rust. One conversation favors
MIR. Another records Avery favoring a higher-level representation because it
preserves loops. His later message explicitly says he does not know whether
BAML's MIR actually contains the difficult kind of control flow.

So we should ask for the prototype and the example that caused trouble.
There is no reason to assume that all MIR is difficult to translate just
because it contains jumps.

The reported small binary and performance near Go are useful leads. We need
the code, build command, workload, and telemetry setting to reproduce them.
They are not a benchmark baseline yet.

A few statements in the notes were too strong. C and Rust can participate in
cross-language optimization with compatible toolchains. Mutation does not
make JIT impossible. A shared heap does not automatically give us working
browser threads. The option notes explain each of these.

The biggest question for Avery is what we must deliver first: a fast native
binary, BAML in a browser, or cheap isolated execution of many programs.
Those goals overlap, but they can favor different designs.

Original notes, relative to this folder:
../../../call_notes.md
../../../hari_notes.md
../../../bamlrust.md

Also used the LLVM and custom-MIR messages pasted into this conversation.
