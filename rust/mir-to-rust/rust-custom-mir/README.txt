Writing Rust MIR directly

Rust has an internal feature that lets us write a function as blocks with
explicit jumps. We could translate BAML MIR into that format and let rustc
handle the remaining compilation. This avoids reconstructing ordinary Rust loops
just to hand them back to a compiler.

The catch is that Rust calls this feature exceptionally unstable. It exists for
rustc's own tests. It requires nightly, supports only certain constructs, and
applies to whole functions. Choosing an early or late insertion point changes
which compiler checks and transformations still run.

BAML MIR and Rust MIR are different representations. We still need to translate
BAML objects, calls, exceptions, and suspension. Garbage collection needs
explicit integration. Sharing the name MIR does not make this a direct
connection.

I ran a small experiment with the installed Rust 1.96.0-nightly from 2026-03-04.
A custom-MIR function contains a loop with two entrances. A second function
expresses the same behavior using ordinary Rust loop/match syntax. Both compiled
and matched an independent arithmetic answer for 2,000 inputs. The source is
probe.rs. This tests a tiny control-flow example, not BAML integration or speed.

Current nightly documentation still carries the instability warning. It
documents jumps, calls, drops, and unwind behavior, but explicitly says not to
assume missing constructs work.

This is worth keeping as an experiment. The unanswered question is whether it
produces materially better code than stable Rust output. If it does not, an
internal compiler dependency buys us little. Compare generated machine code
before committing the project to a pinned nightly.

Checked 2026-09-29 UTC.

probe.rs
https://doc.rust-lang.org/nightly/core/intrinsics/mir/index.html
https://github.com/rust-lang/rust/blob/ea6bb45b74c24a026dab4187b1550e97b6985c8a/library/core/src/intrinsics/mir.rs
