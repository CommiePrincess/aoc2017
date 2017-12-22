fn main() {
    let input = include_str!("input.txt").trim();
    let mut input_grid : Vec<Vec<State>> = Vec::new();

    let mut grid : [[State; 1001]; 1001] = [[State::Clean; 1001]; 1001];
    let (mut current_x, mut current_y) = (1001 / 2, 1001 / 2);
    let mut current_direction = Direction::Up;

    let mut counter = 0;

    for i in input.lines() {
    	input_grid.push(i.chars().map(|x| if x == '#' { State::Infected } else { State::Clean } ).collect());
    }

    let (starting_x, starting_y) = (1001 / 2 - (input_grid.len() / 2), 1001 / 2 - (input_grid.len() / 2));

    for x in 0..input_grid.len() {
    	for y in 0..input_grid.len() {
    		grid[starting_x + x][starting_y + y] = input_grid[y][x];
    	}
    }

    for _ in 0..10000000 {
    	current_direction = turn(current_direction, grid[current_x][current_y]);

    	if grid[current_x][current_y] == State::Flagged {
    		grid[current_x][current_y] = State::Clean;
    	} else if grid[current_x][current_y] == State::Infected {
    		grid[current_x][current_y] = State::Flagged;
    	} else if grid[current_x][current_y] == State::Weakened {
    		grid[current_x][current_y] = State::Infected;
    		counter += 1;
    	} else if grid[current_x][current_y] == State::Clean {
    		grid[current_x][current_y] = State::Weakened;
    	}

    	current_y = (current_y as isize + match current_direction {
    		Direction::Up => -1,
    		Direction::Down => 1,
    		_ => 0,
    	}) as usize;

    	current_x = (current_x as isize + match current_direction {
    		Direction::Left => -1,
    		Direction::Right => 1,
    		_ => 0,
    	}) as usize;
    }

    println!("{}", counter);
}

#[derive(Copy, Clone, PartialEq)]
enum State {
	Clean, 
	Infected,
	Weakened, 
	Flagged
}

enum Direction {
	Up, 
	Down,
	Left,
	Right,
}

fn turn (d: Direction, state: State) -> Direction {
	if state == State::Infected {
		match d {
			Direction::Up => Direction::Right,
			Direction::Right => Direction::Down,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
		}
	} else if state == State::Clean {
		match d {
			Direction::Up => Direction::Left,
			Direction::Left => Direction::Down,
			Direction::Down => Direction::Right,
			Direction::Right => Direction::Up,
		}
	} else if state == State::Weakened {
		d
	} else if state == State::Flagged {
		match d {
			Direction::Up => Direction::Down,
			Direction::Left => Direction::Right,
			Direction::Right => Direction::Left,
			Direction::Down => Direction::Up,
		}
	} else {
		Direction::Up
	}
}
