

pub fn init_search (
    given_input:&Vec<Vec<i32>>,
    _depth_matrix: Vec<Vec<i32>>,
    row_depth: usize
) -> usize {
    let (top_vec, bottom_vec) = (0 as usize, given_input.iter().len() -1 );

    let return_value = 0 as usize;

    for  (_i, rows )in given_input
        .as_slice()[top_vec..bottom_vec+1]
        .into_iter()
        .enumerate()
         {
            let depth = &_depth_matrix[_i];
            for _index in 1..row_depth-1 {
                let current = rows[_index];
                let _up_to = rows.as_slice()[0.._index].into_iter();
                let _to_from = rows.as_slice()[_index+1..row_depth].into_iter();
            }
        }

    return_value
}

pub fn main () {
    let input_data = include_str!("../day_eight/day_eight_input.txt")
        .split("\n")
        .into_iter()
        .collect::<Vec<&str>>();

    let mut parsed_nums:Vec<Vec<i32>> = Vec::new();

    // always assumes the same depth of trees/ rows
    let mut row_depth = 0;

    input_data
        .into_iter()
        .for_each(|x| {
            let mut _row_nums: Vec<i32> = Vec::new();
            for mut _char in x.chars().into_iter() {
                let _char_as_string: String = _char.to_string();
                if _char_as_string.is_empty() {
                    continue;
                }
                _row_nums.push(_char_as_string.parse::<i32>().unwrap());
            }

            if row_depth == 0 {
                row_depth = _row_nums.iter().len();
            }
            if ! _row_nums.is_empty() {
                parsed_nums.push(_row_nums);
            }

        });


    let depth_matrix: Vec<Vec<i32>> = (0..parsed_nums[0].len())
        .map(|i|
            parsed_nums
            .iter()
            .map(|c| c[i])
            .collect::<Vec<i32>>()
        ).collect();

    println!("{:?}", depth_matrix);
    init_search(&parsed_nums, depth_matrix, row_depth as usize);
}