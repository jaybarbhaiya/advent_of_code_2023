mod day1;
mod day2;

#[cfg(test)]
mod tests {
    use crate::{
        day1::print_day1_result,
        day2::{part2::part2, print_valid_game_sum_day2},
    };

    #[test]
    fn day1() {
        print_day1_result();
    }

    #[test]
    fn day2() {
        print_valid_game_sum_day2();
        part2();
    }
}
