use Instruction::*;

fn main() {
    let input : Vec<String> = include_str!("input.txt").trim().split(',').map(|x| String::from(x)).collect();

    let mut instructions : Vec<Instruction> = Vec::new();

    for int in input.iter() {
    	let instruction = int.chars().nth(0).unwrap();

    	if instruction == 's' {
    		instructions.push(Instruction::S(int[1..].parse().unwrap()))
    	} else if instruction == 'p' {
    		instructions.push(Instruction::P((
    			int.chars().nth(1).unwrap(),
    			int.chars().nth(3).unwrap(),
    		)));
    	} else if instruction == 'x' {
    		instructions.push(Instruction::X((
    			int[1..].split("/").nth(0).unwrap().parse().unwrap(),
    			int[1..].split("/").nth(1).unwrap().parse().unwrap()
    		)));
    	}
    }

    let mut programs : Vec<char> = Vec::new();

    for i in 'a' as u32..'q' as u32 {
    	programs.push(std::char::from_u32(i).unwrap());
    }

    let programs_count = programs.len();

    let mut recurring : Vec<Vec<char>> = Vec::new();

    let cycle_length = loop {
    	for int in &instructions {
    		match int {
    			&S (y) => {
    				let spin_slice : Vec<char> = programs.split_off(programs_count - y);

    				for i in spin_slice.into_iter().rev() {
    					programs.insert(0, i);
    				}
    			},
    			&P ((x, y)) => {
    				let i1 = linear_search(&programs, x).unwrap();
    				let i2 = linear_search(&programs, y).unwrap();

    				programs.swap(i1, i2);
    			},
    			&X ((x, y)) => {
    				programs.swap(x, y);
    			},
    		}
    	}

    	if !recurring.contains(&programs) {
    		recurring.push(programs.clone());
    	} else {
    		break recurring.len();
    	}
	};

	for _ in 0..(1000000000 % cycle_length - 1) {
		for int in &instructions {
    		match int {
    			&S (y) => {
    				let spin_slice : Vec<char> = programs.split_off(programs_count - y);

    				for i in spin_slice.into_iter().rev() {
    					programs.insert(0, i);
    				}
    			},
    			&P ((x, y)) => {
    				let i1 = linear_search(&programs, x).unwrap();
    				let i2 = linear_search(&programs, y).unwrap();

    				programs.swap(i1, i2);
    			},
    			&X ((x, y)) => {
    				programs.swap(x, y);
    			},
    		}
    	}
	}

    for p in &programs {
    	print!("{}", p);
    }

    println!();
}

enum Instruction {
	S (usize),
	P ((char, char)),
	X ((usize, usize)),
}

fn linear_search (searching: &[char], target: char) -> Option<usize> {
	for i in 0..searching.len() {
		if searching[i] == target {
			return Some(i);
		}
	}

	None
}
