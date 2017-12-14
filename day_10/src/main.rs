extern crate day_10;

use day_10::KnotHash;

fn main() {
    let input_p1 : Vec<u8> = include_str!("input.txt").trim().split(',').map(|x| x.parse().unwrap()).collect();

    let mut input_p2 : Vec<u8> = include_str!("input.txt").trim().as_bytes().to_vec();
    input_p2.append(&mut vec![17, 31, 73, 47, 23]);

    let mut list_p1 = KnotHash::new(input_p1, 256);
    let mut list_p2 = KnotHash::new(input_p2, 256);

    list_p1.hash_round();

    for _ in 0..64 {
        list_p2.hash_round();
    }

    println!("answer for part 1: {}\nanswer for part 2: {}", list_p1.get_sparse_hash()[0] * list_p1.get_sparse_hash()[1], list_p2.dense_hash());
}
