use::std::collections::HashSet;

pub struct Grid {
    grid: Vec<Vec<char>>,
    width: i32,
    height: i32,
    up: (i32, i32),
    down: (i32, i32),
    right: (i32, i32),
    left: (i32, i32),
}

impl Grid {
    pub fn _new() -> Self {
        Grid {
            grid: Vec::new(),
            width: 0,
            height: 0,
            up: (-1, 0),
            down: (1, 0),
            right: (0, 1),
            left: (0, -1),
        }
    }

    fn _parse(self: &mut Self) {
        let lines = include_str!("../day_eight/day_eight_input.txt")
            .lines()
            .into_iter()
            .map(|s| String::from(s))
            .collect::<Vec<String>>();

        self.width = lines[0].len() as i32;
        self.height = lines.len() as i32;

        for row in lines {
            self.grid.push(row.chars().collect::<Vec<char>>());
        }
    }


    fn _part1 (self: &mut Self) -> i32{
        let mut total:HashSet<(i32, i32)> = HashSet::new();

        for (start, step, search) in [
            ((0,0), self.right, self.down),
            ((0,0), self.down, self.right),
            ((self.height - 1, self.width -1 ), self.up, self.left),
            ((self.height -1, self.width -1), self.left, self.up),
        ]
        {
            let mut walk = start;

            while walk.0 >= 0 && walk.0 < self.height && walk.1 >=0 && walk.1 < self.width {
                let (mut row, mut col) = walk;
                let mut tallest = self.grid[row as usize][col as usize];

                total.insert(walk);

                while tallest < '9' {
                    row += search.0;
                    col += search.1;

                    if row < 0 || row >= self.height || col < 0 || col >= self.width {
                        break;
                    }

                    let tree = self.grid[row as usize][col as usize];
                    if tree > tallest {
                        total.insert((row, col));
                        tallest = tree;
                    }
                }
                walk.0 += step.0;
                walk.1 += step.1;
            }
        }

        total.len() as i32
    }

    fn _part2 () {

    }


}

pub fn main () {
    let mut new_grid = Grid::_new();
    Grid::_parse(&mut new_grid);
    let res = Grid::_part1(&mut new_grid);
    println!("{:?}", res);
}