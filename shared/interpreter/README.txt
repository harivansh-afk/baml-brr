Improving the existing VM

BEX reads an instruction, selects its handler, and performs the operation. That
repeated selection is dispatch. Improving the interpreter keeps the existing
machinery for values, functions, waiting, errors, and garbage collection.

The current compact instruction set has 129 opcodes. It already has specialized
arithmetic such as AddInt and combined movement operations such as LoadVar2. The
meeting note's “255 fast instructions” is not the current count. We should
inspect the actual instruction sequence before proposing a new combined
instruction.

There are two useful questions. Can we execute fewer handlers by combining a
frequent sequence? Can each handler do less work, such as moving fewer values or
avoiding a redundant check? Allocation and container access may matter more than
dispatch, so those belong in the same profile.

Changing dispatch itself is also possible. CPython has an optional interpreter
that transfers between small C opcode handlers using tail calls, which avoid
accumulating call frames. It depends on particular compiler support. That is an
example to study, not a speedup we can assume Rust or BEX will inherit.

The interpreter remains useful even if we choose native compilation. It can
support development and dynamic Package.compile, and provide a reference for
checking generated code. Sharing runtime helpers could let improvements benefit
both paths. Whether a release build can remove the unused interpreter must be
verified from the output.

I would start here for understanding and measurement. Profile representative
programs, choose one frequent sequence, and compare it before and after a small
change. If dispatch dominates, compilation becomes more attractive. If
allocation or synchronization dominates, removing the loop may leave most costs
intact. Keep telemetry and observable behavior identical; one faster loop is not
an application-wide result.

Sources checked 2026-09-29 UTC. Local opcode count: BAML 5956bc4af2,
../../../baml/baml_language/crates/bex_vm_types/src/bytecode.rs, enum OpCode.
https://docs.python.org/3/whatsnew/3.14.html#a-new-type-of-interpreter
https://v8.dev/blog/sparkplug
