mod day1_part1;
mod day2_part2;

#[cfg(test)]
pub(crate) fn read_lines<P>(
    filename: P,
) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufRead::lines(std::io::BufReader::new(file)))
}

#[cfg(test)]
mod tests {
    use crate::day1_part1::print_day1_result;

    #[test]
    fn day1_part1() {
        let result = print_day1_result();
        dbg!(result);
        assert!(result > 0);
    }
}
