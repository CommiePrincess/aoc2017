fn main() {
    let input : Vec<i32> = include_str!("input.txt").lines().map(|l| l.trim().parse().unwrap()).collect();

    println!("answer for part 1: {}\nanswer for part 2: {}", part_1(&input), part_2(&input));
}

fn part_1 (input : &Vec<i32>) -> u32 {
    let mut input : Vec<i32> = input.clone();

    let mut index : usize = 0;
    let mut s = 0;

    while index < input.len() {
        input[index as usize] += 1;

        index = (index as i32 + input[index] - 1) as usize; 

        s += 1;
    }

    s
}

fn part_2 (input : &Vec<i32>) -> u32 {
    let mut input : Vec<i32> = input.clone();

    let mut index : usize = 0;
    let mut s = 0;

    while index < input.len() {
        let offset;

        if input[index as usize] >= 3 {
            offset = -1;
        }else{
            offset = 1;
        }

        input[index as usize] += offset;

        index = (index as i32 + input[index] - offset) as usize; 

        s += 1;
    }

    s
}