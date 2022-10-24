
use std::collections::HashMap;

fn main() {
	println!("{:?}", solution(vec![1,2,7,9,11,15], 15));
}

fn solution(nums: Vec<i32>, target: i32) -> (i32, i32) {
	let mut memory = HashMap::new();

	for (index, val) in nums.iter().enumerate() {
		match memory.get(&(target - val)) {
			Some(&found) => {
				return (found as i32, index as i32);
			},
			None => {}
		}
		memory.insert(val, index);
	}

	return (-1, -1);
}