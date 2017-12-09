use std::collections::HashMap;

fn main() {
	let input = include_str!("input.txt"); 

	let mut registers : HashMap<String, i32> = HashMap::new();

	let mut current_highest : i32 = 0;

	for l in input.lines() {
		let parts : Vec<&str> = l.split_whitespace().collect();

		let register_op = String::from(parts[0]);
		let op = parts[1];
		let change : i32 = parts[2].trim().parse().unwrap();

		let register2 = String::from(parts[4]);
		let cond = parts[5];
		let cond_val : i32 = parts[6].trim().parse().unwrap();

		let comp : Box<Fn(i32, i32) -> bool> = match cond {
			"==" => Box::new(|a: i32, b: i32| -> bool {a == b}),
			"<=" => Box::new(|a: i32, b: i32| -> bool {a <= b}),
			">=" => Box::new(|a: i32, b: i32| -> bool {a >= b}),
			"!=" => Box::new(|a: i32, b: i32| -> bool {a != b}),
			"<"  => Box::new(|a: i32, b: i32| -> bool {a < b}),
			">"  => Box::new(|a: i32, b: i32| -> bool {a > b}),
			_ => Box::new(|_: i32, _: i32| -> bool {false}),
		};

		if comp(*registers.entry(register2).or_insert(0), cond_val) {
			if op == "inc" {
				*registers.entry(register_op.clone()).or_insert(0) += change;
			}else if op == "dec" {
				*registers.entry(register_op.clone()).or_insert(0) -= change;
			}

			if *registers.get(&register_op).unwrap() > current_highest {
				current_highest = *registers.get(&register_op).unwrap();
			}
		}
	}

	println!("answer for part 1: {}\nanswer for part 2: {}", registers.values().max().unwrap(), current_highest);
}
