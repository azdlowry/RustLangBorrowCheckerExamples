#![deny(warnings)]

fn main() {
    let mut x = vec![1, 2, 3];

    for i in &x {
        println!("{}", i);
        //x.push(4);
    }
    println!("{:?}", x);
}
