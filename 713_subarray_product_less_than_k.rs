
//use std::collections::HashMap;

fn main() {

	println!("{:?}", solution(vec![10,5,2,6], 30));

}


fn solution( nums: Vec<i32>, k: i32 ) -> i32 {

	let mut count = 0;
	let size = nums.len();

	for i in 0..size {
		let mut product = nums[i];
		let mut _i = i;

		while product <= k && _i < size {
			count += 1;
			product *= nums[_i];
			_i += 1;
		}
	}

	count
}


// solved for product == k
// fn solution( nums: Vec<i32>, k: i32 ) -> i32 {

// 	let mut count = 0;
// 	let mut prefixProduct :HashMap<String, i32> = HashMap::new();
// 	let mut product = 1f64;

// 	prefixProduct.insert("1".to_string(), 1);

// 	for num in nums {
// 		let div = format!("{}", product/num as f64);
// 		match prefixProduct.get(&div) {
// 			None => {},
// 			Some(&c) => {
// 				count += c;
// 			}
// 		}

// 		product *= num as f64;
// 		let prod = format!("{}", product);
// 		prefixProduct.entry(prod).and_modify(|product| *product += 1).or_insert(1);
// 		println!("{:?}", prefixProduct);
// 	}

// 	count
// }