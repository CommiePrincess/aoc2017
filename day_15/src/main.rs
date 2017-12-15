fn main() {
    let input : Vec<Vec<&str>> = include_str!("input.txt").trim().lines().map(|l| l.split_whitespace().collect()).collect();

    let mut a : u64 = input[0][input[0].len() - 1].parse().unwrap();
    let mut b : u64 = input[1][input[1].len() - 1].parse().unwrap();

    const A_MULT: u64 = 16807;
    const B_MULT : u64 = 48271;
    const REMAINDER_DIV : u64 = 2147483647;

    let mut part_1 = 0;

    for _ in 0..40000000 {
    	a = (a * A_MULT) % REMAINDER_DIV;
    	b = (b * B_MULT) % REMAINDER_DIV;

    	if a & 0xffff == b & 0xffff {
    		part_1 += 1;
    	}
    }

    a = input[0][input[0].len() - 1].parse().unwrap();
    b = input[1][input[1].len() - 1].parse().unwrap();
    
    let mut part_2 = 0;

    for _ in 0..5000000 {
    	a = (a * A_MULT) % REMAINDER_DIV;
    	b = (b * B_MULT) % REMAINDER_DIV;

    	while a % 4 != 0 {
    		a = (a * A_MULT) % REMAINDER_DIV;
    	}

    	while b % 8 != 0{
    		b = (b * B_MULT) % REMAINDER_DIV;
    	}

    	if a & 0xffff == b & 0xffff {
    		part_2 += 1;
    	}
    }

    println!("answer for part 1: {}\nanswer for part 2: {}", part_1, part_2);
}
