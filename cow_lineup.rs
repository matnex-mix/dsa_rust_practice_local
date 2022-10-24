
fn main() {

	println!("{:?}", solution(vec![(25,7), (26, 1), (15, 1), (22, 3), (20, 1), (30, 1)]));

}

fn solution( lineup: Vec<(i32, i32) ) -> i32 {

	lineup.sort_by(|a, b| a.1.cmp(&b.1));

}