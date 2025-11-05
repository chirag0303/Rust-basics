use std::num::ParseIntError;

fn main() {
    let v: Vec<i8> = Vec::new(); // an empty Vector
    let mut v: Vec<u8> = vec![1, 2, 4];
    v.push(10);

    println!("{:?}", v);
    println!("v[1]: {:?}", v.get(1)); // get method return Option data type
    println!("v[1000]: {:?}", v.get(1000)); // This will not give error but v[1000] will give err for  index out of range

    let mut v: Vec<i32> = vec![0; 10];
    println!("v: {:?}", v);

    //update
    v[6] = 100;

    v.pop(); // removes last element
    println!("{:?}", v);

    //slice
    let s = &v[3..7];
    println!("{:?}", s);
}
