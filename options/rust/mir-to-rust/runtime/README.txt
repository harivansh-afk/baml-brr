MIR-to-Rust: the production runtime boundary

Hari's notes ask for runtime behavior shared with the VM. The collector is
already separate; merging crates or generating more scalar code does not by
itself solve that boundary.

Current dependency shape:

  BEX VM ---------> bex_runtime <--------- generated Rust
                       |  \
                       |   bex_heap (optional native-root support)
                       v
                 bex_vm_types -> baml_type

baml_type keeps pure type/integer rules. bex_vm_types keeps values, objects,
panic payloads, and root contracts. bex_heap keeps allocation, collection,
and permits. Those crates stay in place.

Implemented: integer operations and index rules moved to bex_runtime. VM
opcodes/builtins and generated arithmetic now reuse them. Tagged VM add/sub
still use their existing fast path. Generated applications link the runtime
through Cargo instead of copying another implementation.

Implemented separately: native root holders survive real moving GC, preserve
aliases and write barriers, and notify root release. They use the existing heap
manager contract. These tests do not establish generated array support.

Next production slice:

Give each native task one root context. Nested calls share its active permit
and push scoped root frames; do not acquire a permit for each function. Slots
need bounded reuse and generation checks. Transfer returns/errors into caller
roots before clearing callee roots. Root generic type metadata too when added.

Emit int[] allocation, aliasing, indexing, mutation, and length into that
context. The MIR already represents those operations. Begin with one array
passed to a mutating function; verify the caller's alias observes the mutation.
Force collection at allowed boundaries, including across nested calls.

Then define the pause protocol. Save resume position and live scalars; keep
heap references in explicit roots. Poll long-running loops so GC/cancellation
can proceed. Reuse the engine's collector and scheduler, not a second manager.
BexThread currently contains a VM; supporting a native task will require a
shared execution contract below that engine-specific wrapper.

BAML catches, cleanup, traces, reflection, and VM/native callbacks still need
integration. Shared VmPanic data alone does not implement them.

Potential speed improvements are direct operations, native scalar locals,
inlining, and fewer conversions. Extra root traffic, retained references, and
runtime calls can consume those gains. Keep helpers inlineable and preserve
cold error construction, then measure with the instrumentation harness.
