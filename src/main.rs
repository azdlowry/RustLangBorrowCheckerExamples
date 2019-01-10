#![deny(warnings)]

fn main() {
    let y: &Vec<i32>;
    {
        let x = vec![1, 2, 3];
        y = &x;
        println!("{:?}", x);
    }
    println!("{:?}", y);
}
