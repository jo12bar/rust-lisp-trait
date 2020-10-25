mod lisp;

use lisp::prelude::*;
use std::f64::consts::PI;

fn hello_world() {
    println!("Hello, world!");
}

fn run<A, R>(f: impl Fn(A) -> R, a: A) -> R {
    f(a)
}

fn hello(name: String) {
    println!("Hello, {}!", name);
}

fn main() {
    eval((hello_world,));

    let r = 3.0;
    let res = eval((mul, PI, (mul, r, r)));
    println!("{}", res);

    let name = "Sam".to_string();
    eval((run, Box::new(hello), name));
}
