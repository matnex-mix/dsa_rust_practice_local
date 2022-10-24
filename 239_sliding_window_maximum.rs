

fn main() {

	println!("{:?}", solution(vec![1,3,-1,-3,5,3,6,7], 3));

}

fn solution( nums :Vec<i32>, k: i32 ) -> Vec<i32> {

	let mut result :Vec<i32> = vec![];

	let mut left_pointer = 0;
	let mut right_pointer = left_pointer + k;
	let size = nums.len() as i32;

	while right_pointer <= size {

		println!("{}..{}", left_pointer, right_pointer);
		result.push(*nums[left_pointer as usize..right_pointer as usize].iter().max().unwrap());
		
		left_pointer += 1;
		right_pointer += 1;
	}

	result
}