use std::u32;

fn main() {
    let mut x: u32 = u32::MAX;
    x += 1;
    println!("u32 Max {} + 1 = {}", u32::MAX, x);

    let x = u32::checked_add(u32::MAX, 1);
    println!("checked_add: {:?}", x); // returs None or Some

    let x = u32::wrapping_add(u32::MAX, 1);
    println!("wrapping_Add: {:?}", x); // allows overflow
}
