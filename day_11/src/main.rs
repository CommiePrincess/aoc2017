fn main() {
    let input : Vec<&str> = include_str!("input.txt").trim().split(',').collect();

    let mut pos = Position::new(0, 0);

    let mut current_longest : u32 = 0;

    for v in &input {
    	match *v {
    		"ne" => {pos.x += 1; pos.y += 1;},
    		"se" => {pos.x += 1;},
    		"n" => {pos.y += 1;},
    		"s" => {pos.y -= 1;},
    		"nw" => {pos.x -= 1;},
    		"sw" => {pos.x -= 1; pos.y -= 1;},
    		_ => {},
    	}

    	if pos.distance_from_origin() > current_longest {
    		current_longest = pos.distance_from_origin();
    	}
    }

    println!("answer for part 1: {}\nanswer for part 2: {}", pos.distance_from_origin(), current_longest);
}

struct Position {
	x: i32, 
	y: i32,
}

impl Position {
	fn new(x: i32, y: i32) -> Position {
		Position{x, y}
	}

	fn distance_from_origin(&self) -> u32 {
    	let ds = vec![self.x.abs(), self.y.abs(), (self.y - self.x).abs()];

    	*ds.iter().max().unwrap() as u32
	}
}
