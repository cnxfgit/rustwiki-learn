use std::collections::*;
use std::io::{self, Write};
use std::fmt;

fn function1() -> fmt::Result {
    // --略
    fmt::Result::Ok(())
}
fn function2() -> io::Result<()> {
    // --略
    io::Result::Ok(())
}

fn main() {
    let mut map: HashMap<_, _> = HashMap::new();
    map.insert(1, 1);

    let _ = function1();
    let _ = function2();
}
