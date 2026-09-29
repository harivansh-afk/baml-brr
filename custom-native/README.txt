Write our own native backend

This means turning BAML operations into CPU instructions ourselves. We could
compile before deployment or while the program runs. Neither timing choice
requires LLVM, but both require a correct machine-code generator.

The attraction is simplicity for a narrow problem. A small compiler could turn
common BAML operations into fixed instruction sequences and call the Rust
runtime for everything else. We could tailor it to BAML's values and avoid
expensive compiler passes. Those are possible benefits, not measured wins.

The work grows quickly. We must choose instructions, assign values to the CPU's
limited registers, save extra values in memory, pass function arguments
correctly, and produce code that links and can be debugged. Spark's aarch64 and
users' x86-64 machines need different implementations. Operating-system calling
rules differ too.

Existing libraries can supply pieces. regalloc2 handles register allocation:
deciding which values occupy which CPU registers. Assemblers and object-file
libraries can handle encoding and output. We still own how the pieces fit
together and whether the resulting program preserves BAML behavior.

The runtime problems remain. References in registers must survive moving garbage
collection. Shared mutation, errors, suspension, and cancellation need correct
implementations. Calling existing helpers reduces duplicated behavior but may
limit optimization.

Browsers cannot execute our arbitrary native machine code. We would also need
Wasm output or the existing interpreter compiled to Wasm.

I would not start here. Cranelift gives us a way to test native execution
without first building the machinery above. Revisit a custom backend only after
identifying a specific operation existing backends handle poorly, measuring its
importance, and limiting the experiment to that problem. Removing interpreter
dispatch does not by itself establish an advantage over V8.

Sources checked 29 September 2026:
https://llvm.org/docs/CodeGenerator.html
https://github.com/ARM-software/abi-aa/blob/main/aapcs64/aapcs64.rst
https://github.com/bytecodealliance/regalloc2
https://webassembly.org/docs/faq/
