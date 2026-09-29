#![feature(core_intrinsics, custom_mir)]
#![allow(internal_features)]
use core::intrinsics::mir::*;
#[custom_mir(dialect = "built")]
fn direct(n: u32, start: bool) -> u32 {
    mir! {
        let remaining: u32;
        let total: u32;
        {
            remaining = n;
            total = 0;
            match remaining { 0 => done, _ => choose }
        }
        choose = { match start { false => left, _ => right } }
        left = {
            total = total + 1;
            remaining = remaining - 1;
            match remaining { 0 => done, _ => right }
        }
        right = {
            total = total + 2;
            remaining = remaining - 1;
            match remaining { 0 => done, _ => left }
        }
        done = { RET = total; Return() }
    }
}
fn dispatched(n: u32, start: bool) -> u32 {
    let mut remaining = n;
    let mut total = 0;
    let mut block = if start { 1 } else { 0 };
    if remaining == 0 { return total; }
    loop {
        match block {
            0 => { total += 1; remaining -= 1; if remaining == 0 { return total; } block = 1; }
            1 => { total += 2; remaining -= 1; if remaining == 0 { return total; } block = 0; }
            _ => unreachable!(),
        }
    }
}
fn main() {
    for n in 0..1000 {
        for start in [false, true] {
            let expected = 3 * (n / 2) + if n % 2 == 0 { 0 } else if start { 2 } else { 1 };
            assert_eq!(direct(n, start), expected);
            assert_eq!(dispatched(n, start), expected);
        }
    }
    println!("2000 inputs: custom MIR and stable-syntax dispatcher match arithmetic oracle");
}
