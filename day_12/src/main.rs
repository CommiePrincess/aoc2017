use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt").trim();

    let mut nodes : HashMap<u32, Program> = HashMap::new();

    for i in input.lines() {
    	let line : Vec<&str> = i.split_whitespace().collect();
    	let mut p = Program::new(line[0].trim().parse().unwrap());

    	for j in line.iter().skip(2) {
    		let j : u32 = j.replace(',', "").trim().parse().unwrap();

    		nodes.entry(j).or_insert(Program::new(j));

    		p.connected.push(j);
    	}

    	if !nodes.contains_key(&p.id) {
    		nodes.insert(p.id, p.clone());
    	} else {
    		let entry = nodes.entry(line[0].trim().parse().unwrap()).or_insert(Program::new(0));
    		entry.connected = p.connected.clone();
    	}
    }

    let mut p2 : Vec<u32> = nodes.keys().cloned().collect();

    let mut group_count = 0;

    while !p2.is_empty() {
    	let p2_1 = p2.clone();
    	p2.retain(|x| !nodes[&p2_1[0]].get_all_connected(&nodes, vec![]).contains(x));
    	group_count += 1;
    }

    println!("answer for part 1: {}\nanswer for part 2: {}", nodes[&0].get_all_connected(&nodes, vec![]).iter().cloned().collect::<HashSet<u32>>().len(), group_count);
}

#[derive(Clone)]
struct Program {
	id: u32,
	connected: Vec<u32>,
}

impl Program {
	fn new (id: u32) -> Program {
		Program {id, connected: Vec::new() }
	}

	fn get_all_connected (&self, nodes: &HashMap<u32, Program>, current: Vec<u32>) -> Vec<u32> {
		let mut c : Vec<u32> = Vec::new();

		if !current.contains(&self.id) && !c.contains(&self.id) {
			c.push(self.id);
		}

		let c2 = c.clone();

		for i in self.connected.iter().filter(|x| !c2.contains(x) && !current.contains(x)) {
			c.push(nodes[i].id);
		}

		let mut current = current.clone();
		let current2 = current.clone();

		let c2 = c.clone();

		for i in self.connected.iter() {
			for j in nodes[i].connected.iter().filter(|x| !c2.contains(x) && !current2.contains(x)) {
				c.push(*j);
				current.append(&mut c.clone());
				c.append(&mut nodes[i].get_all_connected(nodes, current.clone()).iter().filter(|x| !current.contains(x)).cloned().collect());
			}
		}

		c
	}
}
