fn main() {
    let input : Vec<i32> = include_str!("input.txt").split_whitespace().map(|b| b.parse().unwrap()).collect();

    println!("answer for part 1: {}\nanswer for part 2: {}", part_1(&input), part_2(&input));
}

fn part_1 (input : &Vec<i32>) -> u32 {
	let mut input : Vec<i32> = input.clone();

	let mut previous_patterns : Vec<Vec<i32>> = Vec::new();

	let mut index : usize;

	loop {
		index = input.iter().position(|n| n == input.iter().max().unwrap()).unwrap();

		let mut redist = input[index];
		input[index] = 0;

		while redist > 0 {
			index = (index + 1) % input.len();

			input[index] += 1;
			redist -= 1;
		}

		if previous_patterns.contains(&input) {
			return (previous_patterns.len() + 1) as u32;
		} else {
			previous_patterns.push(input.clone());
		}
	}
}

fn part_2 (input : &Vec<i32>) -> u32 {
	let mut input : Vec<i32> = input.clone();

	let mut previous_patterns : Vec<Vec<i32>> = Vec::new();

	let mut index : usize;

	loop {
		index = input.iter().position(|n| n == input.iter().max().unwrap()).unwrap();

		let mut redist = input[index];
		input[index] = 0;

		while redist > 0 {
			index = (index + 1) % input.len();

			input[index] += 1;
			redist -= 1;
		}

		if previous_patterns.iter().filter(|x| x == &&input).count() == 2 {
			return (previous_patterns.len() - previous_patterns.iter().rposition(|x| x == &input).unwrap()) as u32; 
		} else {
			previous_patterns.push(input.clone());
		}
	}
}