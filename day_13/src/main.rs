use std::collections::HashMap;

fn main() {
	let input = include_str!("input.txt").trim();

	let mut firewall : HashMap<u32, Layer> = HashMap::new();

	for l in input.lines() {
		let values : Vec<u32> = l.split(':').map(|x| x.trim().parse().unwrap()).collect();
	
		firewall.insert(values[0], Layer::new(Some(values[1])));
	}

	let firewall_keys : Vec<u32> = firewall.keys().cloned().collect();

	for i in (0..*firewall.keys().max().unwrap()).filter(|x| !firewall_keys.contains(x)) {
		firewall.insert(i, Layer::new(None));
	}

	let firewall_keys : Vec<u32> = firewall.keys().cloned().collect();

	// part 1

	let mut position : u32 = 0;

	let mut severity = 0;

	while position < firewall.keys().count() as u32 {
		let pos_values = &firewall[&(position as u32)].pos_values;

		if firewall[&position].depth != None && pos_values[(position as usize % pos_values.len()) as usize] == 0 {
			severity += position * firewall[&position].depth.unwrap();
		}

		position += 1;
	}

	println!("answer for part 1: {}", severity);

	// part 2

	let mut delay : usize = 0;

	loop {
		let mut catched = false;
		let mut position : usize = 0;

		while position < firewall_keys.len() {
			let pos_values = &firewall[&(position as u32)].pos_values;

			if firewall[&(position as u32)].depth != None && pos_values[(position + delay) % pos_values.len()] == 0 {
				catched = true;
				break;
			}

			position += 1;
		}

		if !catched {
			break;
		}

		delay += 1;
	}

	println!("answer for part 2: {}", delay);
}

struct Layer {
	depth: Option<u32>,
	pos_values: Vec<i32>,
}

impl Layer {
	fn new (depth : Option<u32>) -> Layer {
		if depth == None {
			return Layer {depth: None, pos_values: vec![]};
		}

		let pos_values : Vec<i32> = (0..depth.unwrap() as i32).chain((1..depth.unwrap() as i32 - 1).rev()).collect();

		Layer {depth: depth, pos_values}
	}
}
