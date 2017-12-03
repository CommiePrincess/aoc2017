fn main() {
    let input : u32 = include_str!("input.txt").trim().parse().unwrap();

    println!("answer for part 1: {}\nanswer for part 2: {}", part_1(input), part_2(input));
}

// defo not an optimal solution, but my initial solution was worse and required me to do some of the calculations by hand, 
// so i decided to use my code from part 2 to solve part 1
fn part_1 (n: u32) -> i32 {
    let mut storage_grid : [[u32; 1001]; 1001] = [[0; 1001]; 1001];

    let starting_point : (usize, usize) = (1001 / 2 + 1, 1001 / 2 + 1);

    storage_grid[starting_point.0][starting_point.1] = 1;

    let mov = |c_pos : &mut (usize, usize), v : (isize, isize)| *c_pos = ((c_pos.0 as isize + v.0) as usize, (c_pos.1 as isize + v.1) as usize);

    let mut position : (usize, usize) = (starting_point.0 + 1, starting_point.1);
    let mut m = 1;

    let mut counter = 2;

    loop {
        let mut s = 1;

        while s <= 8 * m {
            storage_grid[position.0][position.1] = counter;

            if storage_grid[position.0][position.1] == n {
                return (starting_point.0 as i32 - position.0 as i32).abs() + (starting_point.1 as i32 - position.1 as i32).abs();
            }

            if s >= 1 && s < 2 * m {
                mov(&mut position, (0, 1));
            }else if s >= 2 * m && s < 4 * m {
                mov(&mut position, (-1, 0));
            }else if s >= 4 * m && s < 6 * m {
                mov(&mut position, (0, -1));
            }else if s >= 6 * m && s < 8 * m {
                mov(&mut position, (1, 0));
            }

            s += 1;
            counter += 1;
        }

        mov(&mut position, (1, 0));
        m += 1;
    }
}

fn part_2 (n: u32) -> u32 {
    let mut storage_grid : [[u32; 1001]; 1001] = [[0; 1001]; 1001];

    let starting_point : (usize, usize) = (1001 / 2 + 1, 1001 / 2 + 1);

    storage_grid[starting_point.0][starting_point.1] = 1;

    let mov = |c_pos : &mut (usize, usize), v : (isize, isize)| *c_pos = ((c_pos.0 as isize + v.0) as usize, (c_pos.1 as isize + v.1) as usize);

    let mut position : (usize, usize) = (starting_point.0 + 1, starting_point.1);
    let mut m = 1;

    loop {
        let mut s = 1;

        while s <= 8 * m {
            let mut current_count = 0;

            for k in (-1 as i32)..2 {
                for j in (-1 as i32)..2 {
                    if !(k == 0 && j == 0) {
                        let neighbour_x = position.0 as i32 + k;
                        let neighbour_y = position.1 as i32 + j;

                        current_count += storage_grid[neighbour_x as usize][neighbour_y as usize];
                    }
                }
            }

            storage_grid[position.0][position.1] = current_count;

            if storage_grid[position.0][position.1] > n {
                return storage_grid[position.0][position.1];
            }

            if s >= 1 && s < 2 * m {
                mov(&mut position, (0, 1));
            }else if s >= 2 * m && s < 4 * m {
                mov(&mut position, (-1, 0));
            }else if s >= 4 * m && s < 6 * m {
                mov(&mut position, (0, -1));
            }else if s >= 6 * m && s < 8 * m {
                mov(&mut position, (1, 0));
            }

            s += 1;
        }

        mov(&mut position, (1, 0));
        m += 1;
    }
}