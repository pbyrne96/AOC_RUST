mod get_data;

// YOUR OPTIONS
const DEFAULT_ROCK: char = 'X';
const DEFAULT_PAPER: char = 'Y';
const DEFAULT_SCISSORS: char  = 'Z';

// OPPONENTS OPTIONS
const OTHER_ROCK: char = 'A';
const OTHER_PAPER: char = 'B';
const OTHER_SCISSORS: char = 'C';

// DECIDING SYMBOLS
const LOSS: char = 'X';
const DRAW: char = 'Y';
const WON: char = 'Z';

const WON_VALUE: i32 = 6;
const DRAW_VALUE: i32 = 3;
const LOSS_VALUE: i32 = 0;

pub fn find_result (_other: char, _default: char ) -> char {

    return LOSS;
}

pub fn get_result_value (_result: char, _second_char: char ) -> i32 {
    let joined_chars = (_result, _second_char);

    if joined_chars == (OTHER_ROCK, DEFAULT_ROCK) {
        return DRAW_VALUE;
    }
    if joined_chars == (OTHER_ROCK, DEFAULT_PAPER) {
        return WON_VALUE;
    }
    if joined_chars == (OTHER_ROCK, DEFAULT_SCISSORS) {
        return LOSS_VALUE;
    }

    if joined_chars == (OTHER_PAPER, DEFAULT_ROCK) {
        return LOSS_VALUE;
    }
    if joined_chars == (OTHER_PAPER, DEFAULT_PAPER) {
        return DRAW_VALUE;
    }
    if joined_chars == (OTHER_PAPER, DEFAULT_SCISSORS) {
        return WON_VALUE;
    }

    if joined_chars == (OTHER_SCISSORS, DEFAULT_ROCK) {
        return WON_VALUE;
    }
    if joined_chars == (OTHER_SCISSORS, DEFAULT_PAPER) {
        return LOSS_VALUE;
    }

    if joined_chars == (OTHER_SCISSORS, DEFAULT_SCISSORS) {
        return DRAW_VALUE;
    }

    return LOSS_VALUE;
}

pub fn calc_your_score (result: char) -> Result<i32, i32> {
    match result {
        'X' => Ok(1),
        'Y' => Ok(2),
        'Z' => Ok(3),
        _ => Err(0),
    }
}

pub fn run () {
    let _raw_data = get_data::run();
    let ( mut problem_one, mut problem_two) = (0, 0);
    for score_chars in _raw_data.iter() {
       let (first_char, second_char) = (score_chars[0], score_chars[score_chars.iter().len()-1]);
       let first_symbol_score  = calc_your_score(second_char).unwrap();
       let actual_score = first_symbol_score + get_result_value(first_char, second_char);
       problem_one+=actual_score;
       let _decide_symbol = find_result(first_char, second_char);
        problem_two += first_symbol_score + get_result_value(first_char, _decide_symbol);
    }
    println!("{:?}", problem_one);
    println!("{:?}", problem_two);
}