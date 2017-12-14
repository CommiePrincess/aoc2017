use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt").trim();

    let mut firewall : HashMap<u32, Layer> = HashMap::new();

    for l in input.lines() {
    	let values : Vec<u32> = l.split(':').map(|x| x.trim().parse().unwrap()).collect();
    
    	firewall.insert(values[0], Layer::new(Some(values[1])));
    }

    let firewall_keys : Vec<u32> = firewall.keys().cloned().collect();

    for i in (0..*firewall.keys().max().unwrap()).filter(|x| !firewall_keys.contains(x)) {
    	firewall.insert(i, Layer::new(None));
    }

    let mut position : u32 = 0;

    let mut severity = 0;

    let firewall_keys : Vec<u32> = firewall.keys().cloned().collect();

    while position < firewall.keys().count() as u32 {
    	if firewall[&position].current_position == 0 {
    		severity += position * firewall[&position].depth.unwrap();
    	}

 		position += 1;

    	for v in firewall.values_mut() {
    		if v.depth != None {
    			v.current_position = v.pos_values[position as usize % v.pos_values.len()];
    		}
    	}
    }

    println!("answer for part 1: {}", severity);

    let mut delay : usize = 0;

    loop {
    	for v in firewall.values_mut() {
    		if v.depth != None {
    			v.current_position = v.pos_values[delay % v.pos_values.len()];
    		}
    	}

    	severity = 0;
    	let mut position : usize = 0;

    	while position < firewall_keys.len() {
    		if firewall[&(position as u32)].current_position == 0 {
    			severity = 1;
    			break;
    		}

 			position += 1;

    		for v in firewall.values_mut() {
    			if v.depth != None {
    				v.current_position = v.pos_values[(position + delay) % v.pos_values.len()];
    			}
    		}
    	}

    	if severity == 0 {
    		break;
    	}

    	delay += 1;
    }

    println!("answer for part 2: {}", delay);
}

struct Layer {
	current_position: i32,
	depth: Option<u32>,
	pos_values: Vec<i32>,
}

impl Layer {
	fn new (depth : Option<u32>) -> Layer {
		if depth == None {
			return Layer {current_position: -1, depth: None, pos_values: vec![]};
		}

		let mut pos_values : Vec<i32> = (0..depth.unwrap() as i32).collect();

		pos_values.append(&mut (1..depth.unwrap() as i32 - 1).rev().collect());

		Layer {current_position: 0, depth: depth, pos_values}
	}
}
