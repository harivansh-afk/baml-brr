How we evaluate each implementation

Record what changed, what is supported, what is explicitly rejected, and which
existing compiler/runtime operations were reused or duplicated.

Keep the source program, its MIR, generated code, and correctness command
readily available. Record the Git base and compiler version. Describe test
results narrowly enough that another person can reproduce them.

Track engineering effort from actual work, including build problems and
changes needed for GC, errors, callbacks, or suspension. Estimated maintenance
cost is a judgment; record the reason rather than inventing a number.

Performance comes from the separate instrumentation suite. Use matching inputs,
behavior, telemetry, and timing boundaries. Different compiler revisions or
missing runtime work can invalidate an apparent win.

Before extending a prototype, state the next question it will answer. A small
arithmetic result should not silently become a commitment to a full backend.
