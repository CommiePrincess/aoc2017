fn main() {
    let input = include_str!("input.txt");
    let input : Vec<Vec<u32>> = input.lines().map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<u32>>()).collect();

    let calculate_checksum = |row : &Vec<u32>| -> u32 {*row.iter().max().unwrap() - *row.iter().min().unwrap()};

    let spreadsheet_checksum : u32 = input.iter().map(|l| calculate_checksum(&l)).sum();
    let spreadsheet_checksum_2 : u32 = input.iter().map(|l| calculate_checksum_2(&l)).sum();

    println!("solution to part 1: {}\nsolution to part 2: {}", spreadsheet_checksum, spreadsheet_checksum_2);
}

fn calculate_checksum_2(row: &Vec<u32>) -> u32 {
    for n in row.iter() {
        for i in row.iter() {
            if n != i && (n % i == 0) {
                return (n / i) as u32;
            }
        }
    }

    0
}