BAML execution experiments

Start with options/rust/mir-to-rust/README.txt.

Each option has a short README.txt. Once implementation starts, its baml/
folder is a Git worktree with its own branch. We create worktrees as needed;
there is no reason to check out the whole repository for every idea upfront.

options/rust/mir-to-rust/       Current experiment: actual MIR to Rust.
options/rust/checked-to-rust/   Generate Rust before MIR lowering.
options/rust/custom-mir/        Rust's internal MIR interface.
options/c-clang/                Generate C/C++ for Clang.
options/llvm-ir/                Emit LLVM IR.
options/wasm/                   Emit Wasm directly.
options/cranelift/              Use Cranelift's native backend.
options/native/                 Own the native code generator.

execution/ explains AOT and JIT. These describe when compilation happens.
shared/ holds runtime, browser, evidence, and evaluation notes.

The main BAML checkout is ../baml. The first experiment starts at
5956bc4af24521852e6193b1dd5338671f38f51b. The main checkout may advance;
each experiment keeps its recorded base until we deliberately update it.
Use matching revisions when comparing implementations.

Baseline measurements and their runner belong in ../instrumentation.
Experiment code and correctness checks belong in the option's worktree.
Compare performance only after agreeing on inputs, revision, semantics,
telemetry, and measured region.

Open the baml/ folder to inspect or edit code. Run git diff there to see that
experiment's changes. The notes repository ignores these nested worktrees;
it does not track them as submodules.
