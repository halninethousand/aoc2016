#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    facing: Direction,
}

impl Position {
    fn new() -> Self {
        Position {
            x: 0,
            y: 0,
            facing: Direction::North,
        }
    }

    fn walk(&mut self, steps: i32) {
        match self.facing {
            Direction::North => self.y += steps,
            Direction::South => self.y -= steps,
            Direction::East => self.x += steps,
            Direction::West => self.x -= steps,
        }
    }

    fn process_instruction(&mut self, instruction: &str) {
        let direction = instruction.chars().next().unwrap();
        let steps = instruction[1..].parse::<i32>().unwrap();

        match direction {
            'L' => self.facing = self.facing.turn_left(),
            'R' => self.facing = self.facing.turn_right(),
            _ => panic!("Invalid direction"),
        }

        self.walk(steps);
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
   }
}

fn main() {
    let instructions: Vec<&str> = include_str!("../data/day1.txt").split(',').map(|dir| dir.trim()).collect();
    // let instructions = ["R5", "L5", "R5", "R3"];
    let mut position = Position::new();

    for instruction in instructions {
        position.process_instruction(instruction);
        println!("After {}: {:?}", instruction, position);
    }

    println!("taxicab distance: {}", position.manhattan_distance());
}

