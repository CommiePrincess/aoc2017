fn main() {
	let input = include_str!("input.txt");

	let mut is_in_garbage = false;

	let mut score = 0;
	let mut group_level = 0;

	let mut garbage_count = 0;

	let mut input_chars : Vec<char> = input.chars().collect();

	for i in 0..input_chars.len() {
		if !is_in_garbage {
			match input_chars[i] {
				'{' => {
					group_level += 1;
					score += group_level;
				},
				'}' => { group_level -= 1; },
				'<' => { is_in_garbage = true; },   
				_ => {},
			}
		} else {
			match input_chars[i] {
				'{' => { garbage_count += 1; },
				'}' => { garbage_count += 1; },
				'<' => { garbage_count += 1; },
				'>' => { is_in_garbage = false; },  
				'!' => {
					input_chars[i] = ' ';
					input_chars[i + 1] = ' ';
				}, 
				_ => {
					if input_chars[i] != ' ' {
						garbage_count += 1;
					}
				},
			}
		}
	}

	println!("answer for part 1: {}\nanswer for part 2: {}", score, garbage_count);
}
