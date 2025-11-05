pub fn eq(a: char, b: char) -> bool {
    return a == b;
}

pub fn add(x: f32, y: f32, z: f32) -> f32 {
    return x + y + z;
}

pub fn cast(x: u8, y: i8, z: f32) -> f32 {
    x as f32 + y as f32 + z
}
fn main() {
    let result: f32 = cast(2, 98, 4.3);
    println!("{result}");
}
