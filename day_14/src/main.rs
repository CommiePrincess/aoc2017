extern crate day_10;

use std::collections::VecDeque;
use day_10::KnotHash;

fn main() {
    let input = include_str!("input.txt").trim();

    let mut grid : [[u8; 128]; 128] = [[0; 128]; 128];

    for i in 0..128 {
    	let mut hash_input = format!("{}-{}", input, i).as_bytes().to_vec();
    	hash_input.append(&mut vec![17, 31, 73, 47, 23]);
    
    	let mut kh = KnotHash::new(hash_input, 256);

    	for _ in 0..64 {
    		kh.hash_round();
    	}

    	for (index, c) in kh.dense_hash().chars().enumerate() {
    		let val = c.to_digit(16).unwrap();

    		let mut val = format!("{:b}", val);

    		for _ in 0..(4 - val.len()) {
    			val.insert(0, '0');
    		}

    		for (index_2, c) in val.chars().enumerate() {
    			if c == '1' {
    				grid[i][index * 4 + index_2] = 1;
    			}
    		}
    	}
    }

    let mut count = 0;

    let mut to_scan : VecDeque<(u32, u32)> = VecDeque::new();
    let mut regions = 0;

    for x in 0..128 {
    	for y in 0..128 {
    		if grid[x][y] == 1 { 
    			count += 1;
    			to_scan.push_back((x as u32, y as u32));
    		}
    	}
    }

    while !to_scan.is_empty() {
    	scan(to_scan.pop_front().unwrap(), &grid, &mut to_scan);

    	regions += 1;
    }

    println!("answer for part 1: {}\nanswer for part 2: {}", count, regions);
}

fn scan (pos: (u32, u32), grid: &[[u8; 128]; 128], to_scan: &mut VecDeque<(u32, u32)>) {
	if to_scan.contains(&pos) {
		to_scan.retain(|x| *x != pos);
	}

	for i in 0..4 {
		let d = match i {
			0 => (1, 0),
			1 => (-1, 0),
			2 => (0, 1),
			3 => (0, -1),
			_ => (0, 0),
		};

		if (pos.0 as i32 + d.0) >= 0 && (pos.1 as i32+ d.1) >= 0 && to_scan.contains(&(pos.0 + d.0 as u32, pos.1 + d.1 as u32)) {
			scan((pos.0 + d.0 as u32, pos.1 + d.1 as u32), grid, to_scan);
		}
	}
}
