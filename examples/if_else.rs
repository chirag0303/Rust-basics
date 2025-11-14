fn main() {
    let x: i32 = 12;

    let z: i32 = if x > 0 {
        println!("Positive");
        x
    } else if x < 0 {
        println!("Negative");
        x
    } else {
        println!("Zero");
        0
    };
}
