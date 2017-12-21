use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;

fn main() {
	let input : Vec<Vec<&str>> = include_str!("input.txt").trim().lines().map(|x| x.split_whitespace().collect()).collect();

	let instructions1 = input.clone();
	let instructions2 = input.clone();

	let (tx0, rx0) = mpsc::channel();
	let (tx1, rx1) = mpsc::channel();

	let program0 = thread::spawn(move || {
		let mut registers : HashMap<char, i64> = HashMap::new();
		let input = instructions1;

		registers.insert('p', 0);

		let mut index : isize = 0;

		while index >= 0 && index < input.len() as isize {
			let i = &input[index as usize];

			if i[0] == "snd" {
				let arg1 = *registers.entry(i[1].chars().nth(0).unwrap()).or_insert(0);

				tx0.send(arg1).unwrap();
			} else if i[0] == "set" {
				let arg1 = i[1].chars().nth(0).unwrap();
				let arg2 = i[2].parse::<i64>().unwrap_or_else(|_| *registers.entry(i[2].chars().nth(0).unwrap()).or_insert(0));

				registers.insert(arg1, arg2);
			} else if i[0] == "add" {
				let arg1 = i[1].chars().nth(0).unwrap();
				let arg2 = i[2].parse::<i64>().unwrap_or_else(|_| *registers.entry(i[2].chars().nth(0).unwrap()).or_insert(0));

				*registers.entry(arg1).or_insert(0) += arg2;
			} else if i[0] == "mul" {
				let arg1 = i[1].chars().nth(0).unwrap();
				let arg2 = i[2].parse::<i64>().unwrap_or_else(|_| *registers.entry(i[2].chars().nth(0).unwrap()).or_insert(0));

				*registers.entry(arg1).or_insert(0) *= arg2;
			} else if i[0] == "mod" {
				let arg1 = i[1].chars().nth(0).unwrap();
				let arg2 = i[2].parse::<i64>().unwrap_or_else(|_| *registers.entry(i[2].chars().nth(0).unwrap()).or_insert(0));

				*registers.entry(arg1).or_insert(0) %= arg2;
			} else if i[0] == "rcv" {
				let arg1 = i[1].chars().nth(0).unwrap();

				let value = rx1.recv().unwrap();

				registers.insert(arg1, value);
			} else if i[0] == "jgz" {
				let arg1 = i[1].parse::<i64>().unwrap_or_else(|_| *registers.entry(i[1].chars().nth(0).unwrap()).or_insert(0));
				let arg2 = i[2].parse::<i64>().unwrap_or_else(|_| *registers.entry(i[2].chars().nth(0).unwrap()).or_insert(0));

				if arg1 > 0 {
					index += arg2 as isize;
					continue;
				}
			}

			index += 1;
		}
	});

	let program1 = thread::spawn(move || {
		let mut registers : HashMap<char, i64> = HashMap::new();
		let input = instructions2;

		let mut sent_count = 0;

		registers.insert('p', 1);

		let mut index : isize = 0;

		while index >= 0 && index < input.len() as isize {
			let i = &input[index as usize];

			if i[0] == "snd" {
				let arg1 = *registers.entry(i[1].chars().nth(0).unwrap()).or_insert(0);

				tx1.send(arg1).unwrap();

				sent_count += 1;
				println!("{}", sent_count);
			} else if i[0] == "set" {
				let arg1 = i[1].chars().nth(0).unwrap();
				let arg2 = i[2].parse::<i64>().unwrap_or_else(|_| *registers.entry(i[2].chars().nth(0).unwrap()).or_insert(0));

				registers.insert(arg1, arg2);
			} else if i[0] == "add" {
				let arg1 = i[1].chars().nth(0).unwrap();
				let arg2 = i[2].parse::<i64>().unwrap_or_else(|_| *registers.entry(i[2].chars().nth(0).unwrap()).or_insert(0));

				*registers.entry(arg1).or_insert(0) += arg2;
			} else if i[0] == "mul" {
				let arg1 = i[1].chars().nth(0).unwrap();
				let arg2 = i[2].parse::<i64>().unwrap_or_else(|_| *registers.entry(i[2].chars().nth(0).unwrap()).or_insert(0));

				*registers.entry(arg1).or_insert(0) *= arg2;
			} else if i[0] == "mod" {
				let arg1 = i[1].chars().nth(0).unwrap();
				let arg2 = i[2].parse::<i64>().unwrap_or_else(|_| *registers.entry(i[2].chars().nth(0).unwrap()).or_insert(0));

				*registers.entry(arg1).or_insert(0) %= arg2;
			} else if i[0] == "rcv" {
				let arg1 = i[1].chars().nth(0).unwrap();

				let value = rx0.recv().unwrap();

				registers.insert(arg1, value);
			} else if i[0] == "jgz" {
				let arg1 = i[1].parse::<i64>().unwrap_or_else(|_| *registers.entry(i[1].chars().nth(0).unwrap()).or_insert(0));
				let arg2 = i[2].parse::<i64>().unwrap_or_else(|_| *registers.entry(i[2].chars().nth(0).unwrap()).or_insert(0));

				if arg1 > 0 {
					index += arg2 as isize;
					continue;
				}
			}

			index += 1;
		}
	});

	program0.join().unwrap();
	program1.join().unwrap();
}
