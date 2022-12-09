use::std::collections::{HashSet};


// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

impl  Direction {
    fn _parse(s:&str) -> i32 {
        match s {
            "U" => 0,
            "D" => 1,
            "L" => 2,
            "R" => 3,
            _ => panic!("could not find direction"),
        }
    }
}

#[derive(Debug)]
struct Positions {
    _head: (i32, i32),
    _tail: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

const _DIRS: [(i32, i32); 4] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
];

impl Positions {

    pub fn _new () -> Self {
        Positions { _head: (0,0), _tail: (0, 0), visited: HashSet::new() }
    }

    pub fn _make_move (self: &mut Self, _dir: &i32) -> () {
        let delta = _DIRS[*_dir as usize];
        self._head.0 += delta.0;
        self._head.1 += delta.1;

        let row_diff = self._head.0 - self._tail.0;
        let col_diff = self._head.1 - self._tail.1;

        if row_diff == 0 && col_diff.abs() > 1 {
            self._tail.1 += col_diff.signum();

        } else if col_diff == 0 && row_diff > 1 {
            self._tail.0 += col_diff.signum();

        } else if row_diff.abs() > 1 && col_diff.abs() > 1 {
            self._tail.0 += row_diff.signum();
            self._tail.1 += col_diff.signum();

        }

    }
}

pub struct  DayEight {
    steps: Vec<(i32, i32)>,
}

impl DayEight {
    pub fn _new() -> Self {
        DayEight {
            steps: Vec::new()
        }
    }

    pub fn _parse_input (self: &mut Self) {
        let input = include_str!("../d9/d9_input.txt")
            .lines()
            .into_iter()
            .collect::<Vec<&str>>();

        for line in input {
            let (dir, dis) = line.split_once(' ').unwrap();
            let dir = Direction::_parse(dir);
            let distance = dis.parse::<i32>().unwrap();
            self.steps.push((dir, distance));
        }
    }

    pub fn _part_one (self: &mut Self) -> i32 {
        let mut current_positions = Positions::_new();

        for (dir, amount) in &self.steps {
            for _  in 0..*amount {
                current_positions._make_move(dir);
            }
        }

        current_positions.visited.len() as i32
    }
}

pub fn main () {

    let mut day_eight = DayEight::_new();
    day_eight._parse_input();
    day_eight._part_one();

}