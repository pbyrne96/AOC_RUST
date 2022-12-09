mod attempt_two;


pub fn _slice_depth_arr (arr_to_slice: &Vec<Vec<i32>>, pos: usize) -> (Vec<i32>, Vec<i32>) {
    let mut depth_arr:Vec<i32> = Vec::new();
    for arr in arr_to_slice.into_iter() {
        depth_arr.push(arr[pos]);
    }
    (
        depth_arr.as_slice()[0..pos].to_vec(),
        depth_arr.as_slice()[pos+1..].to_vec()
    )
}

pub fn _init_search (
    given_input:&Vec<Vec<i32>>,
    row_depth: usize
) -> i32 {
    let (top_vec, bottom_vec) = (0 as usize, given_input.iter().len() -1 );
    let return_value = 0;

    for  (_i, rows )in given_input
        .as_slice()[top_vec..bottom_vec+1]
        .into_iter()
        .enumerate()
         {
            for _index in 1..row_depth-1 {
                let _compare_val = rows[_index];
                let (_up_to, _to_from) = (
                    rows.as_slice()[0.._index].to_vec(), rows.as_slice()[_index+1..row_depth].to_vec()
                );
                let (above, below) = _slice_depth_arr(given_input, _index);
                println!("{:?}, {:?}", above, below);
            }
        }
    return_value
}

pub fn main () -> Result<(u32, u32), ndarray::ShapeError> {
    let input_data = include_str!("../day_eight/day_eight_input.txt")
        .split("\n")
        .into_iter()
        .collect::<Vec<&str>>();

    let mut parsed_nums:Vec<Vec<i32>> = Vec::new();

    // always assumes the same depth of trees/ row

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

            if ! _row_nums.is_empty() {
                parsed_nums.push(_row_nums);
            }

        });


    let take_two = attempt_two::day8(include_str!("../day_eight/day_eight_input.txt"));
    println!("{:?}", take_two);

    // let row_depth = parsed_nums[0].len();
    // let _c = init_search(&parsed_nums, row_depth as usize);

    Ok(take_two)

}