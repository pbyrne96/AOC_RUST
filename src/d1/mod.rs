mod read_file;
mod problem_one;
mod problem_two;
mod imporved_solution;

pub fn run () {
    let _file: Vec<i32> = read_file::run();
    // let problem_one = problem_one::run(_file.to_vec());
    // let problem_two = problem_two::run(_file.to_vec());
    // println!("{:?}", problem_one);
    println!("{:?}", _file);

}