use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt").trim();

    let mut nodes : HashMap<u32, Program> = HashMap::new();

    for i in input.lines() {
        let line : Vec<&str> = i.split_whitespace().collect();
        let mut p = Program::new(line[0].trim().parse().unwrap());

        p.connected = line.iter().skip(2).map(|x| x.replace(',', "").trim().parse().unwrap()).collect();

        nodes.insert(p.id, p.clone());
    }

    let mut p2 : VecDeque<u32> = nodes.keys().cloned().collect();

    let mut group_count = 0;

    while !p2.is_empty() {
        let to_scan = p2.pop_front().unwrap();
        nodes[&to_scan].scan(&nodes, &mut p2);

        group_count += 1;
    }

    println!("answer for part 1: {}\nanswer for part 2: {}", nodes[&0].get_group(&nodes).len(), group_count);
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

    fn get_group (&self, nodes: &HashMap<u32, Program>) -> HashSet<u32> {
        let mut queue : VecDeque<u32> = VecDeque::from(vec![self.id]);
        let mut visited : HashSet<u32> = HashSet::new();

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            visited.insert(current);

            for c in &nodes[&current].connected {
                if !visited.contains(c) {
                    queue.push_back(*c);
                }
            }
        }

        visited
    }

    fn scan (&self, nodes: &HashMap<u32, Program>, queue: &mut VecDeque<u32>) {
        if queue.contains(&self.id) {
            queue.retain(|x| *x != self.id);
        }

        for i in &self.connected {
            if queue.contains(&i) {
                nodes[&i].scan(nodes, queue);
            }
        }
    }
}
