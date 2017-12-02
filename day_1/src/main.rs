fn main() {
    let input = include_str!("input.txt");

    let input : Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let part_1 = |input_vec: &[u32]| (0..(input_vec.len())).filter(|i| input_vec[*i] == input_vec[(i + 1) % input_vec.len()]).map(|e| input_vec[e]).sum::<u32>();
    let part_2 = |input_vec: &[u32]| (0..(input_vec.len() / 2)).filter(|i| input_vec[*i] == input_vec[i + (input_vec.len() / 2)]).map(|e| input_vec[e] * 2).sum::<u32>();

    println!("solution to part 1: {}, solution to part 2: {}", part_1(&input), part_2(&input));
}