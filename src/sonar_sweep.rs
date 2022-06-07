use std::fs::File;
use std::io::Read;

pub(crate) fn sonar_sweep() {
    let mut input: Vec<i32> = Vec::new();

    read_input(&mut input);

    println!("{}", parse_input(input));
}

fn parse_input(input: Vec<i32>) -> i32{
    let mut count: i32 = 0;
    for i in 1..input.len() {
        if input[i - 1] < input[i] {
            count += 1;
        }
    }
    count
}

fn read_input(input: &mut Vec<i32>) {
    let mut file = File::open("sonar_sweep_input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");
    for line in contents.lines() {
        input.push(line.parse::<i32>().unwrap());
    }
}