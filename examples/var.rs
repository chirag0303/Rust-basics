#![allow(unused)]

const NUM: i32 = 10;
fn main() {
    let x = 1;

    let x: _ = 1233; // placeholder tell rust to automatically give type

    println!("{}", x);
    println!("{x}"); //for simple variables

    println!("{0} + {0} = {1}", x, x + x); //position arguments
    println!("{NUM}");

    //debug
    println!("DEBUG: x {:?}", x);
    println!("DEBUG: x {:#?}", x); //for multiline variable
}
