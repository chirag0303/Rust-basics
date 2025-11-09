#![allow(unused)]
fn main() {
    let x =10;
    match x{
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("others"),
    }

    match x{
        1| 2| 3 => println!("One or Two or Three"),
        _ => println!("Others"),
    }

    match x{
        i @ 1..11 => println!("One to Ten {i}"), // assigns to i if this case is true
        _ => println!("Others"),
    }

    let x: Option<i32> = Some(8);
    let z = match x{
        Some(val) => val,
        None => 0,
    };
    println!("match returned {z}");

    let res: Result<i32, String> = Ok(1234);
    let res: Result<i32, String> = Err("Error".to_string());
    match res{
        Ok(val) => println!("Result value {val}"),
        Err(err) => println!("Err: {err}"),
    }
}