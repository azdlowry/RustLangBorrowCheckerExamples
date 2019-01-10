#![deny(warnings)]

fn main() {
    let mut x = 23;
    let mut y = &mut x;

    inc(&mut y);
    println!("{}", y);

    inc(&mut x);
    println!("{}", x);

    let z = &x;
    println!("{}", z);
}

fn inc(x: &mut i32) {
    *x += 1;
}
