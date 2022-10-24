use std::collections::{HashSet, HashMap};

fn main() {

	println!("{:?}", solution(vec![(25,7), (26, 1), (15, 1), (22, 3), (20, 1), (30, 1)]));

}

fn solution( mut lineup: Vec<(i32, i32)> ) -> i32 {

	let mut breeds = HashSet::new();
	let mut checked = HashMap::new();

	let mut count = i32::MAX;
	let mut start = 0;

	lineup.sort_by(|a, b| {
		breeds.insert(a.1);
		breeds.insert(b.1);

		a.0.cmp(&b.0)
	});

	let breed_count = breeds.len();

	breeds.clear();

	for end in 0..lineup.len() {
		//println!("AT POINT {}", end);
		checked.entry(lineup[end].1).and_modify(|c| *c += 1).or_insert(1);
		//println!("{:?}", checked);

		// if it is not good, increase the end
		if checked.len() != breed_count {
			continue;
		}
		// if it is good then increase start
		else {
			//println!("start increasing the start pointer");
			while checked.len() == breed_count && start < end {
				match checked.get_mut(&lineup[end].1) {
					Some(c) => {
						if c > &mut 1 {
							*c -= 1;
							start += 1;
						}
						else {

							//println!("end {}, start {}", end, start);

							let _count = lineup[end].0 - lineup[start].0;
							if _count < count {
								count = _count;
							}

							break;
						}
					},
					None => {
						break;
					},
				}

				//println!("{}", start);
				//println!("{:?}", checked);
			}
		}
	}

	//println!("result: {}", count);
	count as i32
}