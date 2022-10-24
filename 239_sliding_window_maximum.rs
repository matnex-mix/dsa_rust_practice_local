

fn main() {

	println!("{:?}", solution(vec![1,3,-1,-3,5,3,6,7], 3));

}

fn solution( nums :Vec<i32>, k: i32 ) -> Vec<i32> {
    let mut result :Vec<i32> = vec![];

    let mut left_pointer :usize = 0;
    let mut right_pointer :usize = left_pointer + k as usize;
    let size = nums.len();

    while right_pointer <= size {
        match result.last() {
            Some(&max_before) => {
                // if what we're adding is greater than the previous maximum then we're to use that
                let adding = nums[right_pointer - 1];

                if adding >= max_before {
                    result.push(adding);
                    //println!("Adding the new element");
                }
                // if what we're cutting of from the window is less than the maximum, then it's safe to use the previous max
                else if left_pointer > 0 && nums[left_pointer - 1] < max_before {
                    //println!("Adding the previous max");
                    result.push(max_before);
                }
                else {
                	//println!("Iterating");
                    result.push(*nums[left_pointer..right_pointer].iter().max().unwrap());
                }
            },
            None => {
                //println!("Iterating");
                result.push(*nums[left_pointer..right_pointer].iter().max().unwrap());
            },
        };

        left_pointer += 1;
        right_pointer += 1;
    }

    result
}