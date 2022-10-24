
use std::collections::HashMap;

fn main() {
	println!("{:?}", solution_sliding_pointers(vec![1,2,7,9,11,15], 11 ));
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

fn solution_sliding_pointers(mut nums: Vec<i32>, target: i32) -> (i32, i32) {
	nums.sort();

	let mut first_pointer = 0;
	let mut second_pointer = (nums.len() - 1) as i32;
	let mut sum = 0;

	while first_pointer < second_pointer {
		sum = nums[first_pointer as usize] + nums[second_pointer as usize];
		if sum > target {
			second_pointer -= 1;
		}
		else if sum < target {
			first_pointer += 1;
		}
		else {
			return (first_pointer, second_pointer);
		}
	}

	return (-1, -1);
}