fn main() {
    let input : Vec<Vec<char>> = include_str!("input.txt").lines().map(|x| x.chars().collect::<Vec<char>>()).collect();
    
    let mut position : (usize, usize) = (0, input[0].iter().enumerate().filter(|x| *x.1 == '|').nth(0).unwrap().0);
    let mut direction : Direction = Direction::Down;

    let mut letters : Vec<char> = Vec::new();

    let mut steps = 0;

    while input[position.0][position.1] != ' ' {
    	position = match direction {
    		Direction::Up => (position.0 - 1, position.1),
    		Direction::Down => (position.0 + 1, position.1), 
    		Direction::Left => (position.0, position.1 - 1),
    		Direction::Right => (position.0, position.1 + 1),
    	};

    	if input[position.0][position.1] == '+' {
    		match direction {
    			Direction::Up | Direction::Down => {
    				if input[position.0][position.1 - 1] == '-' {
    					direction = Direction::Left;
    				} else {
    					direction = Direction::Right;
    				}
    			},
    			Direction::Left | Direction::Right => {
    				if input[position.0 - 1][position.1] == '|' {
    					direction = Direction::Up;
    				} else {
    					direction = Direction::Down;
    				}
    			},
    		}
    	} else if input[position.0][position.1] != '|' && input[position.0][position.1] != '-' {
    		letters.push(input[position.0][position.1]);
    	}

    	steps += 1;
    }

    println!("{}", letters.into_iter().collect::<String>());

    println!("{:?}", steps);
}

enum Direction {
	Up,
	Down,
	Left,
	Right,
}
