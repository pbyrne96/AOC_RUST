use std::fs;

pub fn run () -> Vec<Vec<char>> {
    let mut return_vec: Vec<Vec<char>> = Vec::new();
    let _raw_data = fs::read_to_string("./d2/day_two_input.txt")
        .expect("Read the data");

    let collected_raw_data = _raw_data.lines().into_iter();
    for line in collected_raw_data {

        let line_chars = line
            .chars()
            .into_iter()
            .collect::<Vec<char>>();
        return_vec.push(line_chars);
    }

    return return_vec;
}