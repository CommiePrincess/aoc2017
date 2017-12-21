use std::collections::HashSet;

fn main() {
    let input : Vec<Vec<&str>> = include_str!("input.txt").trim().lines().map(|x| x.split(',').map(|y| y.trim()).collect::<Vec<&str>>()).collect();

    let mut input : Vec<Particle> = input.iter().map(|vec| {
    	let x0 : i64 = vec[0][3..].parse().unwrap();
    	let y0 = vec[1].parse().unwrap();
    	let z0 = vec[2][..vec[2].len() - 1].parse().unwrap();
    	let pos_vector = Vector::new(x0, y0, z0);

    	let x1 : i64 = vec[3][3..].parse().unwrap();
    	let y1 = vec[4].parse().unwrap();
    	let z1 = vec[5][..vec[5].len() - 1].parse().unwrap();
    	let vel_vector = Vector::new(x1, y1, z1);

    	let x2 : i64 = vec[6][3..].parse().unwrap();
    	let y2 = vec[7].parse().unwrap();
    	let z2 = vec[8][..vec[8].len() - 1].parse().unwrap();
    	let acc_vector = Vector::new(x2, y2, z2);

    	Particle::new(pos_vector, vel_vector, acc_vector)
    }).collect();

    for _ in 0..1000 {
    	let mut collision_positions : HashSet<Vector> = HashSet::new();

		for p in &mut input {
    		p.update();
    	}

    	for p0 in input.iter() {
    		for p1 in input.iter() {
    			if p0 != p1 && p0.pos == p1.pos {
    				collision_positions.insert(p0.pos.clone());
    				break;
    			}
    		}
    	}

    	input.retain(|x| !collision_positions.contains(&x.pos));
    }

    let mut shortest : (usize, u64) = (0, 999999999999);

    for i in input.iter().enumerate() {
    	if i.1.largest_distance_from_origin < shortest.1 {
    		shortest.0 = i.0;
    		shortest.1 = i.1.largest_distance_from_origin;
    	}
    }

    println!("{:?}", shortest.0);
    println!("{}", input.len());
}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Particle {
	pos: Vector,
	vel: Vector,
	acc: Vector,
	largest_distance_from_origin: u64,
}

impl Particle {
	fn new (pos: Vector, vel: Vector, acc: Vector) -> Particle {
		Particle {pos, vel, acc, largest_distance_from_origin: 0}
	}

	fn update(&mut self) {
		self.vel.add(&self.acc);
		self.pos.add(&self.vel);

		if self.pos.distance_from_origin() > self.largest_distance_from_origin {
			self.largest_distance_from_origin = self.pos.distance_from_origin();
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
struct Vector {
	x: i64,
	y: i64,
	z: i64,
}

impl Vector {
	fn new (x: i64, y: i64, z: i64) -> Vector {
		Vector {x, y, z}
	}

	fn distance_from_origin (&self) -> u64 {
		(self.x.abs() + self.y.abs() + self.z.abs()) as u64
	}

	fn add (&mut self, other: &Vector) {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
	}
}
