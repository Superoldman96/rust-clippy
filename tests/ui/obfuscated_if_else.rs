#![warn(clippy::obfuscated_if_else)]
#![allow(clippy::unnecessary_lazy_evaluations)]

fn main() {
    true.then_some("a").unwrap_or("b");
    true.then(|| "a").unwrap_or("b");

    let a = 1;
    (a == 1).then_some("a").unwrap_or("b");
    (a == 1).then(|| "a").unwrap_or("b");

    let partial = (a == 1).then_some("a");
    partial.unwrap_or("b"); // not lint
}
