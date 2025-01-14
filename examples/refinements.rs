#![allow(unused)]

#[flux_rs::sig(fn(bool[true]))]
fn assert(_: bool) {}

#[flux_rs::sig(fn(x: i32) -> i32[x + 1])]
fn incr(x: i32) -> i32 {
    x + 1
}

fn test() {
    assert(incr(1) <= 2); // ok
    assert(incr(2) <= 2); // fail
}
