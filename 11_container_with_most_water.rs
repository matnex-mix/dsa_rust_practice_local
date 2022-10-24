 
 fn main() {
 	
 	println!("{:?}", solution(vec![1,3,8,6,9,8,3,6,3,4]));

 }

 fn solution( coordinates :Vec<i32> ) -> i32 {
 	let (mut left_pointer, mut right_pointer) = (0, coordinates.len() as i32 - 1);

 	let mut max_area_so_far = 0;
 	let mut area = 0;

 	while left_pointer < right_pointer {
 		let (h1, h2) = (coordinates[left_pointer as usize], coordinates[right_pointer as usize]);

 		if h1 < h2 {
 			area = h1 * (right_pointer - left_pointer);
 			left_pointer += 1;
 		}
 		else {
 			area = h2 * (right_pointer - left_pointer);
 			right_pointer -= 1;
 		}

 		if area > max_area_so_far {
 			max_area_so_far = area;
 		}
 	}

 	max_area_so_far
 }