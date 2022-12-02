use std::fs;

pub fn run () -> Vec<Vec<char>> {
    let mut return_vec: Vec<Vec<char>> = Vec::new();
    let _raw_data = fs::read_to_string("./day_two/day_two_input.txt")
        .expect("Read the data");

    let collected_raw_data = _raw_data.lines().into_iter();
    for line in collected_raw_data {

        let mut line_chars = line
            .chars()
            .into_iter()
            .collect::<Vec<char>>();

        let empty_index = line_chars.iter().position(|x| *x == ' ' );
        if Some(empty_index) == None {
            line_chars.remove(empty_index.unwrap());
        }
        return_vec.push(line_chars);
    }

    return return_vec;
}