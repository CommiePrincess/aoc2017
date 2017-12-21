use std::collections::HashSet;

fn main() {
	let mut image : Vec<Vec<bool>> = vec![vec![false, true, false], vec![false, false, true], vec![true, true, true]];
	let mut rules : Vec<Rule> = Vec::new();

    let input = include_str!("input.txt").trim();

    for i in input.lines() {
    	let (pattern, result) = (i.split("=>").nth(0).unwrap().trim(), i.split("=>").nth(1).unwrap().trim());

    	let pattern_vec : Vec<Vec<bool>> = pattern.split('/').map(|i| i.chars().map(|c| c == '#').collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();
    	let replace_vec : Vec<Vec<bool>> = result.split('/').map(|i| i.chars().map(|c| c == '#').collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();

    	rules.push(Rule::new(pattern_vec.clone(), replace_vec.clone()));
    }

    for n in 0..18 {
    	let mut new_grid : Vec<Vec<bool>> = Vec::new();

    	let m = if image.len() % 2 == 0 {
    		2 
    	} else {
    		3
    	};

    	for _ in 0..((image.len() / m) * (m + 1)) {
    		new_grid.push(Vec::new());
    	}

    	for x in 0..(image.len() / m) {
    		for y in 0..(image.len() / m) {
    			let mut scan_grid : Vec<Vec<bool>> = Vec::new();

    			for i in 0..m {
    				scan_grid.push(image[y * m + i][(x * m)..(x * m + m)].to_vec());
    			}

    			let result = &rules.iter().filter(|r| r.patterns.contains(&scan_grid)).nth(0).unwrap().result;

    			for i in 0..result.len() {
    				let mut l = result[i].clone();
    				new_grid[y * (m + 1) + i].append(&mut l);
    			}
    		}
    	}

    	image = new_grid;

    	if n == 4 || n == 17 {
    		let count : usize = image.iter().map(|x| x.iter().filter(|y| **y).count()).sum();

    		println!("after {} iterations: {}", n + 1, count);
    	}
    }
}

fn flip (to_flip: &[Vec<bool>]) -> Vec<Vec<bool>> {
	to_flip.iter().map(|r| r.iter().rev().cloned().collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>()
}

fn transpose (to_transp: &[Vec<bool>]) -> Vec<Vec<bool>> {
	to_transp.iter().enumerate().map(|(i, r)| r.iter().enumerate().map(|(j, _)| to_transp[j][i]).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>()
}

fn rot_90 (to_rot: &[Vec<bool>]) -> Vec<Vec<bool>> {
	transpose(to_rot).into_iter().rev().collect::<Vec<Vec<bool>>>()
}

#[derive(Debug)]
struct Rule {
	patterns: HashSet<Vec<Vec<bool>>>,
	result: Vec<Vec<bool>>,
}

impl Rule {
	fn new (pattern: Vec<Vec<bool>>, result: Vec<Vec<bool>>) -> Rule {
		let mut pattern = pattern;
		let mut patterns = HashSet::new();

		for _ in 0..4 {
			patterns.insert(pattern.clone());
			patterns.insert(flip(&pattern));
			pattern = rot_90(&pattern);
		}

		Rule {result, patterns}
	}
}
