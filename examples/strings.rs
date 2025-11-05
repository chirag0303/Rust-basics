#![allow(unused)]

pub fn hello() -> String {
    return String::from("Hello Rust");
}

pub fn greet(name: &str) -> String {
    format!("Hello {}", name) // for adding two Strings 
}

pub fn append(mut s: String) -> String {
    s += "!";
    return s;
}
// String and &str are different
fn main() {
    let s: String = String::from("Hello");
    let new: String = append(s);
    println!("{new}");

    let s: &str = &new[..2]; //creating a string slice
    println!("{}", greet(s));
}
