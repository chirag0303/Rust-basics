fn mul(x: u32, y: u32) -> u32 {
    return x * y;
}

fn div(x: u32, y: u32) -> u32 {
    x / y // withour return keyword and semicolon
}

fn main() {
    let x = 78;
    let y = 35;
    let product = mul(x, y);
    let div = div(x, y);
    println!("{x} * {y} = {product}");
    println!("{x} / {y} = {div}");
}
