use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub(crate) fn drive() {
    let mut input: Vec<i64> = Vec::new();

    read_input(&mut input, None);
    //mimic_input(&mut input);


    println!("{}", solve(input));
}

fn solve(input: Vec<i64>) -> i64 {
    let mut count: i64 = 0;
    let mut count_horizontal: i64 = 0;
    for i in 0..input.len() {
        if input[i] > 10 {
            count_horizontal += input[i] / 10;
        } else {
            count += input[i];
        }
    }
    count * count_horizontal
}

fn mimic_input(input: &mut Vec<i64>) {
    read_input(input, Option::Some(String::from("drive_input_test.txt")));
}

fn read_input(input: &mut Vec<i64>, location: Option<String>) {
    let sign_map: HashMap<String, i64> = HashMap::from([
        // not the cleanest, but if i want the input to only be a vector, i can do this (you can observe that the input is always < 10)
        (String::from("forward"), 10),
        (String::from("down"), 1),
        (String::from("up"), -1),
    ]);
    let mut file = File::open(location.unwrap_or(String::from("drive_input.txt"))).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");
    for line in contents.lines() {
        let mut split = line.split_whitespace();
        input.push(sign_map[split.next().unwrap()] * split.next().unwrap().parse::<i64>().unwrap());
    }
}