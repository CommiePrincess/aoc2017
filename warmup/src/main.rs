#[derive(Debug)]
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

    fn distance (&self, point: &Position) -> i32 {
        ((self.x - point.x).abs() + (self.y - point.y).abs())
    }
}

#[derive(Debug)]
enum Marker {
    A,
    B,
    Null,
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

    {
        let mut farthest_from_origin : &Position = &origin;

        for marker in &markers {
            if marker.distance(&origin) > farthest_from_origin.distance(&origin) {
                farthest_from_origin = &marker;
            }
        }

        println!("{:?}, with distance {}", farthest_from_origin, farthest_from_origin.distance(&origin));
    }

    {
        let mut farthest_pair : (&Position, &Position) = (&origin, &origin);

        let (a_markers, b_markers) : (Vec<&Position>, Vec<&Position>) = markers.iter().partition(|x| match x.marker_type {Marker::A => true, _ => false});

        a_markers.iter().for_each(|a| b_markers.iter().for_each(|b| {
            if &a.distance(&b) > &farthest_pair.0.distance(&farthest_pair.1) {
                farthest_pair = (&a, &b);
            }
        }));

        println!("{:?}, with distance {}", farthest_pair, farthest_pair.0.distance(farthest_pair.1));
    }
}