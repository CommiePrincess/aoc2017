fn main() {
    let input : usize = include_str!("input.txt").trim().parse().unwrap();

    let mut buffer : Vec<u32> = vec![0];
    let mut position : usize = 0;

    let mut last_insertion_index = 0;

    for i in 1..2018 {
    	position = (position + input) % buffer.len();

    	buffer.insert(position + 1, i);
    	position += 1;

    	if i == 2017 {
    		last_insertion_index = position;
    	}
    }

    let mut part_2 = 0;
    position = 0;

    for i in 1..50000001 {
    	position = (position + input) % i;

    	if position == 0 {
    		part_2 = i;
    	}
    	
    	position += 1;
    }

    println!("answer for part 1: {}\nanswer for part 2: {}", buffer[last_insertion_index + 1], part_2);
}
