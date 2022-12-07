use::std::fs;
use::std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

const SKIP_COMMANDS : [&str; 3] = [
    "$ ls",
    "$ cd ..",
    "$ cd /",
];

lazy_static!{
    pub static ref SWITCH_DIR: Regex = Regex::new(r"cd\s[a-z|..|/]").unwrap();
    pub static ref IS_DIR: Regex = Regex::new(r"dir\s[a-zA-z]").unwrap();
    pub static ref DIR_NAME: Regex = Regex::new(r"cd\s[a-zA-Z]").unwrap();
    pub static ref BACK_DIR: Regex = Regex::new(r"").unwrap();
}

const SEPARATOR: &str = ":";
const ROOT: &str = "root";

pub fn match_regex (string_to_match: &str) -> bool  {
    return Some(SWITCH_DIR.find(string_to_match)).unwrap() != None;
}

pub fn is_dir (string_to_match: &str) -> bool {
    return Some(IS_DIR.find(string_to_match)).unwrap() != None;
}

pub fn is_dir_name (string_to_match: &str) -> bool {
    return Some(DIR_NAME.find(string_to_match)).unwrap() != None;
}

pub fn is_back_dir (string_to_match: &str) -> bool {
    return Some(BACK_DIR.find(string_to_match)).unwrap() != None;
}


pub fn process_commands<'a> (holding_matches: &'a mut String, last_dir: &'a mut Vec<&String> ) -> Vec<&'a String> {

    holding_matches.pop();

    let iter_holding_matches = holding_matches.split(SEPARATOR).into_iter().collect::<Vec<&str>>();

    let mut key = ROOT;
    if is_back_dir(key) {

        key = &last_dir.pop().unwrap();
    } else if is_dir_name(key) {
        key = key.split_whitespace().into_iter().collect::<Vec<&str>>().pop().unwrap();
    }

    println!("{:?}, {:?}", key, iter_holding_matches);

    for line in iter_holding_matches {
        if SKIP_COMMANDS.contains(&line) || line.is_empty() {
            continue;
        }

        if is_dir(line) {
            continue;
        }

        // println!("{:?}", line);

    }

    return last_dir.to_vec();

}

fn running_total (data: &Vec<&str>) {
    let mut totals_over_x: HashMap<&str, usize> = HashMap::new();
    totals_over_x.insert(ROOT, 0);

    let mut holding_matches = String::new();
    let binding = String::from(ROOT);
    let mut last_dir:Vec<&String> = [&binding].to_vec();

    for cmds in data {
        if  match_regex(&cmds) {
            last_dir = process_commands(&mut holding_matches, &mut last_dir);
            holding_matches = String::new();
        };

        holding_matches.push_str(cmds.trim());
        holding_matches.push_str(SEPARATOR);
    }

    if !holding_matches.is_empty() {
        process_commands(&mut holding_matches, &mut last_dir);
    }

    drop(holding_matches);
}


pub fn main () {
    let binding = fs::read_to_string("./day_seven/day_seven.txt")
        .expect("Read the data");
    let data: Vec<&str> = binding
        .lines()
        .into_iter()
        .collect::<Vec<&str>>();

    running_total(&data);
}