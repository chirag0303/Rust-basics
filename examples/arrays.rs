#![allow(unused)]
fn main() {
    let arr: [u32; 3] = [3, 5, 76];
    println!("arr[0] = {}", arr[0]);

    let mut arr: [u32; 3] = [3, 5, 76];
    arr[1] = 99;

    let arr: [u32; 10] = [0; 10]; //array with 10 zeroes
    println!("Array: {:?}", arr);

    let nums: [u32; 10] = [2, 4, 6, 3, 78, 54, 2, 79, 90, 4];

    // Slicing
    let s: &[u32] = &nums[..3]; // first 3 index - 0,1,2 (excluding 3)
    let s: &[u32] = &nums[7..]; //last 3 index
    let s: &[u32] = &nums[4..7]; //middle 3 elements 
    println!("{:?}", s);
}
