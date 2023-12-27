use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_two_digit_seq_from_lines(lines: io::Lines<BufReader<File>>, seq: &mut Vec<String>) {
    for line in lines {
        let mut digits = vec![];
        if let Ok(ip) = line {
            if let Some(first_digit) = ip.chars().find(|c| c.is_digit(10)) {
                digits.push(first_digit)
            }

            if let Some(last_digit) = ip.chars().rev().find(|c| c.is_digit(10)) {
                digits.push(last_digit)
            }

            let two_digit_seq: String = digits.iter().collect();
            seq.push(two_digit_seq);
        }
    }
}

fn calculate_sum_of_seq(seq: Vec<String>) -> u32 {
    let mut total = 0;
    for num_str in seq {
        let num = num_str
            .parse::<u32>()
            .expect("Invalid number, could not parse");
        total += num;
    }
    total
}

pub fn print_day1_result() -> u32 {
    let mut seq: Vec<String> = vec![];
    let lines = read_lines("./src/day1/input.txt").expect("Cannot read file");
    extract_two_digit_seq_from_lines(lines, &mut seq);
    calculate_sum_of_seq(seq)
}
