use std::collections::HashMap;

fn main() {

    //hashmaps is like dictionary in python
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("blue"), 100);
    scores.insert(String::from("red"), 700);

    println!("{:#?}", scores);

    let r = scores.get("red");
    let g = scores.get("green");
    println!("{:?} {:?}", r,g); 

    let red: &mut i32 = scores.entry("blue".to_string()).or_insert(0);
    *red += 100;
    let black: &mut i32 = scores.entry("black".to_string()).or_insert(400);

    println!("{:#?}", scores);
}
