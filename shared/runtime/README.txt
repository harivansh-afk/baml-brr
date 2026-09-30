What we still need after removing the VM loop

The runtime is the support code a program needs while it runs. For BAML, that
includes storing objects, collecting unused memory, running tasks, handling
errors, and making requests to the outside world.

Generating Rust does not remove those jobs. Consider two variables pointing
to the same array. Updating it through one variable must still be visible
through the other. Making two independent Rust arrays would change the result.

BAML's garbage collector also moves objects. If generated code keeps an array
address, then pauses while collection happens, that address may be wrong when
it resumes. The runtime needs to know which references are still in use and
update them. A Rust local containing a pointer does not arrange this by itself.

That gives us an early experiment: create an array, keep it alive, allow a
collection, then read it again. Try the same thing while a task is suspended,
and with two variables sharing the array.

The MIR-to-Rust worktree now has bex_runtime. BEX and generated arithmetic
share its operations and panic payloads; array index rules moved there too.
The existing bex_heap and bex_vm_types remain separate shared foundations.

A native-root holder is tested against actual moving GC, including aliases,
write barriers, and release notification. It does not yet give the compiler
scoped native call frames or resumable execution. See the production plan:
../../options/rust/mir-to-rust/runtime/README.txt

There are also two different performance changes here. We can remove the work
of reading bytecode while retaining BAML's current values. Later we can try
storing known integers or floats directly in generated code. Testing those
separately tells us where any improvement came from.

Source files, relative to this folder:
../../../baml/baml_language/crates/bex_vm_types/src/roots.rs
../../../baml/baml_language/crates/bex_heap/src/gc.rs
../../../baml/baml_language/crates/bex_vm/src/package_baml/mod.rs
../../../baml/baml_language/crates/bex_vm_types/src/types/value.rs
