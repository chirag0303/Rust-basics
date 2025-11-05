#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point3D(i32, i32, i32); // diff way to create a struct

struct Empty;
#[derive(Debug)]
struct Circle {
    //nested struct
    radius: i32,
    center: Point,
}

fn main() {
    let p: Point = Point { x: 1, y: 5 };
    println!("{:?}", p);

    let p2: Point3D = Point3D(2, 5, 7);
    println!("{:?}", p2);

    let emptyStruct = Empty;

    let circle = Circle {
        radius: 10,
        center: Point { x: 10, y: 20 },
    };
    println!("{:#?}", circle);

    let x = 2;
    let y = 4;
    let p: Point = Point { x, y }; // shortcut - variables name are same    

    // copy methds
    let p0 = Point { x: 3, y: 10 };
    let p1 = Point { y: 34, ..p0 };
    println!("p1 copy: {:?}", p1);

    let mut p: Point = Point { x: 23, y: 12 };
    p.x += 1;
    p.y = 67;
    println!("Updated: {:?}", p);
}
