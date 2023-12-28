#[cfg(test)]
pub fn print_day1_result() {
    use std::fs::read_to_string;

    let mut p1 = 0;
    let mut p2 = 0;

    let contents = read_to_string("./src/day1/input.txt").expect("Cannot read file");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    for line in lines {
        let mut p1_digits = vec![];
        let mut p2_digits = vec![];

        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                p1_digits.push(c);
                p2_digits.push(c);
            } else {
                for (index, num_str) in [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .into_iter()
                .enumerate()
                {
                    if line[i..].starts_with(num_str) {
                        p2_digits.push(char::from_digit((index + 1) as u32, 10).unwrap());
                    }
                }
            }
        }

        p1 += (p1_digits[0].to_digit(10).unwrap() * 10)
            + p1_digits[p1_digits.len() - 1].to_digit(10).unwrap();

        p2 += (p2_digits[0].to_digit(10).unwrap() * 10)
            + p2_digits[p2_digits.len() - 1].to_digit(10).unwrap();
    }

    println!("day 1 part 1 result: {}", p1);
    println!("day 2 part 2 result: {}", p2);
}
