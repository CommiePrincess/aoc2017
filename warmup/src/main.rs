enum Marker {
    A,
    B,
    Null,
}

struct Position {
    x : i32,
    y : i32,
    marker_type : Marker,
}

impl Position {
    fn mov (&mut self, dir : (i32, i32)) {
        self.x += dir.0;
        self.y += dir.1;
    }

    fn distance (&self, point: &Position) -> u32 {
        ((self.x - point.x).abs() + (self.y - point.y).abs()) as u32
    }
}

fn main() {
    let mut markers : Vec<Position> = Vec::new();
    let mut current_pos = Position {x: 0, y: 0, marker_type: Marker::Null};

    let origin = Position {x: 0, y: 0, marker_type: Marker::Null};

    let instructions = include_str!("input.txt");

    for word in instructions.split(',').map(|w| w.trim()) {
        match word {
            "Start" => break,
            "A" => markers.push(Position{x: current_pos.x, y: current_pos.y, marker_type: Marker::A}),
            "B" => markers.push(Position{x: current_pos.x, y: current_pos.y, marker_type: Marker::B}),
            "Up" => current_pos.mov((0, 1)),
            "Down" => current_pos.mov((0, -1)),
            "Right" => current_pos.mov((1, 0)),
            "Left" => current_pos.mov((-1, 0)),
            _ => (),
        } 
    }

    let part_1_answer : u32 = markers.iter().map(|m| m.distance(&origin)).max().unwrap() as u32;

    let mut longest_distance : u32 = 0;

    let (a_markers, b_markers) : (Vec<&Position>, Vec<&Position>) = markers.iter().partition(|x| match x.marker_type {Marker::A => true, _ => false});

    a_markers.iter().for_each(|a| b_markers.iter().for_each(|b| {
        if a.distance(b) > longest_distance {
            longest_distance = a.distance(b);
        }
    }));

    let part_2_answer : u32 = longest_distance;

    println!("answer for part 1: {}\nanswer for part 2: {}", part_1_answer, part_2_answer);
}