pub struct KnotHash {
    input: Vec<u8>,
    sparse_hash: Vec<u32>,
    knot_hash: Option<String>,
    current_position: usize,
    skip_size: usize,
}

impl KnotHash {
    pub fn new (input: Vec<u8>, length: u32) -> KnotHash {
        KnotHash {input, sparse_hash: (0..length).collect(), knot_hash: None, current_position: 0, skip_size: 0}
    }

    pub fn hash_round (&mut self) {
        for l in self.input.iter() {
            let list_length : usize = self.sparse_hash.len();

            for n in 0..(*l as f32 / 2.0).ceil() as usize {
                self.sparse_hash.swap((self.current_position + n) % list_length, (self.current_position + *l as usize - n - 1) % list_length);
            }

            self.current_position += *l as usize + self.skip_size;
            self.skip_size += 1;
        }
    }

    pub fn get_sparse_hash (&self) -> &Vec<u32> {
        &self.sparse_hash
    }

    pub fn dense_hash (&self) -> String {
        let mut dense_hash : Vec<u8> = Vec::new();

        for c in self.sparse_hash.chunks(16) {
            let mut current_xor = c[0];

            for j in 1..16 {
                current_xor ^= c[j];
            }

            dense_hash.push(current_xor as u8);
        }

        let dense_hash_str : String = dense_hash.iter().map(|c| {
            let mut c = format!("{:x}", c);

            if c.len() == 1 { c.insert(0, '0') }

            c
        }).collect();
        
        dense_hash_str
    }

    pub fn full_hash (&mut self) -> String {
        if self.knot_hash == None {
            for _ in 0..64 {
                self.hash_round();
            }

            self.knot_hash = Some(self.dense_hash());
        }

        self.knot_hash.clone().unwrap()
    }
}
