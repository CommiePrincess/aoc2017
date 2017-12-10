fn main() {
    let input_p1 : Vec<u8> = include_str!("input.txt").trim().split(',').map(|x| x.parse().unwrap()).collect();

    let mut input_p2 : Vec<u8> = include_str!("input.txt").trim().as_bytes().to_vec();
    input_p2.append(&mut vec![17, 31, 73, 47, 23]);

    let mut list_p1 = HashList::new(256);
    let mut list_p2 = HashList::new(256);

    list_p1.hash_round(&input_p1);

    for _ in 0..64 {
        list_p2.hash_round(&input_p2);
    }

    println!("answer for part 1: {}\nanswer for part 2: {}", list_p1.sparse_hash[0] * list_p1.sparse_hash[1], list_p2.dense_hash());
}

struct HashList {
    sparse_hash: Vec<u32>,
    current_position: u32,
    skip_size: u32,
}

impl HashList {
    fn new (length: u32) -> HashList {
        HashList {sparse_hash: (0..length).collect(), current_position: 0, skip_size: 0}
    }

    fn hash_round (&mut self, lengths: &Vec<u8>) {
        for l in lengths.iter() {
            let list_length : u32 = self.sparse_hash.len() as u32;
            let mut values : Vec<u32> = Vec::new();

            for n in self.current_position..self.current_position + *l as u32 {
                values.push(self.sparse_hash[(n % (self.sparse_hash.len() as u32)) as usize]);
            }

            values.reverse();

            for n in self.current_position..self.current_position + *l as u32 {
                self.sparse_hash[(n % (list_length as u32)) as usize] = values[n as usize - self.current_position as usize];
            }

            self.current_position += *l as u32 + self.skip_size;
            self.skip_size += 1;
        }
    }

    fn dense_hash (&self) -> String {
        let mut dense_hash : Vec<u8> = Vec::new();

        for i in 0..self.sparse_hash.len() / 16 {
            let mut current_xor = self.sparse_hash[(i * 16)];

            for j in 1..16 {
                current_xor = current_xor ^ self.sparse_hash[i * 16 + j];
            }

            dense_hash.push(current_xor as u8);
        }

        let mut dense_hash_str = String::new();

        for i in dense_hash.iter() {
            let mut c = format!("{:x}", i);

            if c.len() == 1 { c.insert(0, '0') }

            dense_hash_str += &c;
        }
        
        dense_hash_str
    }
}
