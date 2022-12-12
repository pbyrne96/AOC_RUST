
#[derive(Debug)]
enum CommandsCheck {
    Noop,
    Addx(i32)
}

pub struct Cpu {
    _commands: Vec<CommandsCheck>,
}


impl Cpu {
    pub fn _new () -> Self {
        Cpu {
            _commands: Vec::new() ,
        }
    }

    pub fn parse_input (self: &mut Self)  {
        let buffer = include_str!("../d10/day_ten_input.txt")
            .lines();

        for line in buffer.into_iter() {
            let words = line.split_whitespace().collect::<Vec<&str>>();
            self._commands.push(
                match words[0] {
                    "noop" => CommandsCheck::Noop,
                    "addx" => CommandsCheck::Addx(words[1].parse::<i32>().unwrap()),
                    _ => panic!("unknown string"),
                }
            );
        }

    }

    pub fn _parse_cycles (self: &mut Self) -> i32 {
        let mut strength = 0;
        let mut xreg = 1;
        let mut cycle = 0;
        let mut check = 20;

        for s in &self._commands {
            match s {
                CommandsCheck::Noop => {
                    cycle += 1;
                    if cycle >= check {
                        strength += check* xreg;
                        check+=40;
                    }
                },
                CommandsCheck::Addx(val) => {
                    cycle += 2;
                    if cycle >= check {
                        strength += check * xreg;
                        check += 40;
                    }
                    xreg += val;
                },
                _ => panic!("pancking no number"),
            }
        }

    strength
    }

}


pub fn main () {
    let mut day_ten = Cpu::_new();
    day_ten.parse_input();
    println!("{:?}", day_ten._parse_cycles());
}