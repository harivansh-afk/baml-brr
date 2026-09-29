What the existing evidence establishes

The current compiler exposes actual MIR plus checked type and call information.
The emitter experiments can use those APIs; no new parser is required.

The hand-written custom-MIR probe and ordinary Rust dispatcher matched an
arithmetic oracle for 2,000 inputs. This proves that particular example works,
not that a general BAML translator is correct or fast.

The snapshot audit found no irreducibility in 55 functions, but only two had
cycles and implicit exception edges were absent. BAML's general control-flow
shape remains an open question.

The option READMEs cite upstream capabilities such as LLVM control-flow repair,
C/Rust LTO, Cranelift code generation, and Wasmtime precompilation. Those facts
do not establish BAML integration or a performance ranking.

Evidence:
../../options/rust/custom-mir/probe.rs
../../options/rust/mir-to-rust/control-flow/snapshot-audit.json
../evidence/README.txt
