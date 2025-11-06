pub fn sum(nums: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for i in nums{
        sum+=i;
    }
    sum
}

pub fn fill(x: u32, n: usize) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    for j in 0..n{
        v.push(x);
    }
    return v;
}
