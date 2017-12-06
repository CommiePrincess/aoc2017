use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("answer for part 1: {}\nanswer to part 2: {}", part_1(input), part_2(input));
}

fn part_1 (input : &str) -> u32 {
    let mut count = 0;

    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let mut non_duplicate_words : Vec<&str> = Vec::new();

        let mut count_inc = 1;

        for w in words.iter() {
            if !non_duplicate_words.contains(&w) {
                non_duplicate_words.push(w);
            } else {
                count_inc = 0;
                break;
            }
        }
        count += count_inc;
    }

    count
}

fn part_2 (input : &str) -> u32 {
    let mut count = 0;

    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let mut words_characters : Vec<HashMap<char, u32>> = Vec::new();

        let mut status : bool = true;

        for word in words.iter() {
            let mut word_contents : HashMap<char, u32> = HashMap::new();

            word.chars().for_each(|c| {
                let input_char = word_contents.entry(c).or_insert(0);
                *input_char += 1;
            });

            words_characters.push(word_contents);
        }

        for i in 0..words_characters.len() {
            for j in 0..words_characters.len() {
                if i != j && words_characters[i] == words_characters[j] {
                    status = false;
                    break;
                }
            }

            if !status {
                break;
            }
        }

        if status {
            count += 1;
        }
    }

    count
}