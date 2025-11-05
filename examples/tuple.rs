pub fn first(t: (bool, u32, char)) -> bool {
    t.0
}

pub fn last(t: (bool, u32, char)) -> char {
    let (_, _, c) = t; // partial destructuring
    return c;
}

pub fn swap(t: (u32, u32)) -> (u32, u32) {
    let (a, b) = t;
    return (b, a);
}
fn main() {
    let t: (u32, u32) = (20, 50);
    let (x, y) = swap(t);

    let t = (true, 34, 'C');
    let c = last(t);

    println!("{x},{y},{c}");
}
