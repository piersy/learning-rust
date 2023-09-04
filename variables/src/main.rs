// const x: i32 = 7;

use std::process::exit;

fn main() {
    let x: i32 = 5;
    let y = 4;

    let f = x;
    match x.checked_add(y) {
        Some(val) => val,
        None => exit(1),
    };

    // wrapping_add(x, 5);
    println!("X is:{x}");
    println!("X is:{f}");

    let s = "hello";
    let g = &s[0..3];
    println!("X is:{g}");
    // x = 6;
    // println!("X is:{r}");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
}
