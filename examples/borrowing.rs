//Immuntable references - can be many at a time
// Mutable references - only one read & write access to a value at a time.
// Either mutable or immutable reference at a time
// Reference must not outlive the value. (in scope)
fn main() {
    let mut s: String = String::from("rust");
    let s1 = &mut s;
    //let s2 = &s;
    // let s2 = &mut s; // This will give error

    s1.push_str("Hi! ");

    {
        let s3 = s; // here, ownership is transferred
    }
    // println!("{s1}") // after that we are printing the reference so error
}
