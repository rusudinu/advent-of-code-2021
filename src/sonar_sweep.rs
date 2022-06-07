use std::fs::File;
use std::io::Read;

pub(crate) fn sonar_sweep() {
    let mut input: Vec<i32> = Vec::new();

    read_input(&mut input);
    //mimic_input(&mut input);


    println!("{}", parse_input_part2(input));
}

fn parse_input_part2(input: Vec<i32>) -> i32 {
    let mut count: i32 = 0;
    let measurement_range = 3;
    for i in 0..input.len() - measurement_range {
        let mut a_sum: i32 = 0;
        let mut b_sum: i32 = 0;
        for j in 0..measurement_range {
            a_sum += input[i + j];
            b_sum += input[i + j + 1];
        }
        if a_sum < b_sum {
            count += 1;
            println!("{} {} {} (increased)", input[i], a_sum, b_sum);
        } else {
            println!("{} {} {} (decreased)", input[i], a_sum, b_sum);
        }
    }
    count
}

fn parse_input(input: Vec<i32>) -> i32 {
    let mut count: i32 = 0;
    for i in 1..input.len() {
        if input[i - 1] < input[i] {
            count += 1;
        }
    }
    count
}

fn mimic_input(input: &mut Vec<i32>) {
    input.push(199);
    input.push(200);
    input.push(208);
    input.push(210);
    input.push(200);
    input.push(207);
    input.push(240);
    input.push(269);
    input.push(260);
    input.push(263);
}

fn read_input(input: &mut Vec<i32>) {
    let mut file = File::open("sonar_sweep_input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");
    for line in contents.lines() {
        input.push(line.parse::<i32>().unwrap());
    }
}