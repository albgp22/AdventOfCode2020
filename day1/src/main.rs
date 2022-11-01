use std::io::{self, BufRead};
use std::collections::HashSet;

fn find_two_numbers(nums: &[i32], sum: i32){
    for num in nums{
        if nums.contains(&(sum-num)){
            println!(
                "{},{},{}",
                num, sum-num,
                num*(sum-num),
            );
        }
    }
}

fn find_three_numbers(nums: &[i32], sum: i32){
    for i in 0..(nums.len()-2){
        let mut s: HashSet<i32> = HashSet::new();
        let curr_sum = sum-nums[i];
        for j in i..nums.len(){
            if s.contains(&(curr_sum-nums[j])){
                println!(
                    "{},{},{},{}", 
                    nums[i], nums[j], curr_sum-nums[j],
                    nums[i]*nums[j]*(curr_sum-nums[j]),
                );
            }
            s.insert(nums[j]);
        }
    }
}

fn main() {
    let nums: Vec<i32> = io::stdin().lock().lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i32>().unwrap())
        .collect();
    find_two_numbers(&nums, 2020);
    find_three_numbers(&nums, 2020);

}
