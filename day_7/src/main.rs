use std::collections::HashMap;

fn main() {
    let input : Vec<Vec<&str>> = include_str!("input.txt").lines().map(|l| l.split_whitespace().collect::<Vec<&str>>()).collect();

    let mut nodes : HashMap<String, Program> = HashMap::new();
    let mut children : HashMap<String, Vec<String>> = HashMap::new();

    for line in &input {
        nodes.insert(String::from(line[0]), Program::new(String::from(line[0]), line[1].replace(|c| c == '(' || c ==')', "").parse().unwrap()));
        children.insert(String::from(line[0]), line.iter().skip(3).map(|s| s.replace(",", "")).collect());
    }

    let children_vec : Vec<String> = children.values().flat_map(|x| x.iter()).cloned().collect();

    let root_node = nodes.iter().filter(|&(name, _)| !children_vec.contains(name)).nth(0).unwrap().1;
    let tree = Program::new_with_children(root_node.name.clone(), root_node.weight, &nodes, &children);

    println!("answer for part 1: {}\nanswer for part 2: {}", tree.name, tree.find_imbalance(tree.sum).unwrap().1);
}

struct Program {
    name: String,
    weight: u32,
    sum: u32,
    children: Vec<Program>,
}

impl Program {
    fn new (name: String, weight: u32) -> Program{
        Program {name, weight, children: Vec::new(), sum: 0}
    }

    fn new_with_children (name: String, weight: u32, nodes_map: &HashMap<String, Program>, child_map: &HashMap<String, Vec<String>>) -> Program{
        let mut p = Program::new(name, weight);
        p.find_children(nodes_map, child_map);
        p.sum = p.tree_sum();
        p
    }

    fn find_children (&mut self, nodes_map: &HashMap<String, Program>, child_map: &HashMap<String, Vec<String>>) {
        for c in &child_map[&self.name]  {
            self.children.push(Program::new_with_children(nodes_map[c].name.clone(), nodes_map[c].weight, nodes_map, child_map));
        }
    }

    fn tree_sum (&self) -> u32{
        let mut sum : u32 = self.weight;

        for child in &self.children {
            sum += child.tree_sum();
        }

        sum
    }

    fn find_imbalance(&self, ideal_self_sum: u32) -> Option<(&Program, u32)> {
    	let desired_sum = self.children.iter().map(|c| c.sum).min().unwrap();

    	if self.children.iter().filter(|c| c.sum != desired_sum).count() > 0 {
    		self.children.iter().filter(|c| c.sum != desired_sum).nth(0).unwrap().find_imbalance(desired_sum)
    	} else {
    		let children_total_sum : u32 = self.children.iter().map(|c| c.sum).sum(); 
    		Some((self, ideal_self_sum - children_total_sum))
    	}
    }
}
