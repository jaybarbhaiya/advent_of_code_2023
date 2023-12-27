mod day1;

#[cfg(test)]
mod tests {
    use crate::day1::print_day1_result;

    #[test]
    fn day1() {
        let result = print_day1_result();
        dbg!(result);
        assert!(result > 0);
    }
}
