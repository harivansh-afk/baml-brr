Generate C or C++

Here we turn BAML MIR into C source and let Clang or GCC compile it. C has
labels and goto, so MIR blocks can translate directly without reconstructing
Rust's nested loops. C++ offers the same escape, but jumping around object
initialization and destruction needs extra care. Plain C is the simpler starting
point.

The existing Rust runtime can stay. Generated code calls a small interface for
allocating objects, accessing fields, and performing host operations. Both sides
must agree on arguments and memory layouts. Arbitrary Rust containers cannot
simply be treated as C structures.

The notes say this boundary prevents link-time optimization. That is incorrect.
Rust documents cross-language optimization with C-compatible dependencies. Clang
and Rust must produce compatible LLVM inputs, use the same optimization mode,
and link through suitable tooling. Ordinary GCC object files do not provide that
LLVM path. Whether small helper calls disappear still needs inspection.

C also has different arithmetic rules. We must deliberately implement BAML's
overflow, shifts, division, and error behavior rather than assume a C operator
means the same thing. Objects that move during garbage collection need
registered references and refreshed pointers, just as with the other routes.

Clang can emit Wasm, making this potentially usable for both native and browser
builds. Browser scheduling and host services remain separate runtime work.

This is worth considering if direct block translation saves substantial effort.
Generate the same small function as C and Rust, call the same runtime helper,
and compare with and without cross-language optimization. Add forced collection
before trusting the interface. I see no demonstrated reason yet to prefer C++,
or evidence that either beats Rust output.

Sources checked 29 September 2026:
https://www.gnu.org/software/c-intro-and-ref/manual/html_node/goto-Statement.html
https://doc.rust-lang.org/rustc/linker-plugin-lto.html
https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
https://lld.llvm.org/WebAssembly.html
