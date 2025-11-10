fn main() {
    let i = 1; // This will be stoored in stack
    let i = Box::new(1i32);

    //Ownership rule
    //1. Every value must have a owner
    let s: String= String::from("Hello"); // owner is 's'

    //2. Only one owner at at time, prev owner get dropped
    let s1= s;
    println!("{s1}");  
    //println!("{s}"); // This will give error beacause s is dropped

    //3. When owner goes out of scope, value gets dropped

    let s= "hhello".to_string();
    {
        s;
    } // scope ends
    //println!("{s}"); // This will give error

    // All the ownership rules are not applicabble to data types that implement copy trait like i32.
}