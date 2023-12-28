pub mod part2;

#[cfg(test)]
pub(crate) fn print_valid_game_sum_day2() {
    use std::fs::read_to_string;

    use regex::Regex;

    let mut valid_games_sum = 0;

    let mut games: Vec<String> = read_to_string("./src/day2/game_input.txt")
        .expect("Could not open file")
        .lines()
        .map(String::from)
        .collect();
    let re = Regex::new(r"Game \d+: ").unwrap();
    for i in 0..games.len() {
        let mut valid_game = true;
        games[i] = re.replace_all(&games[i], "").to_string();
        let rounds: Vec<String> = games[i].split(';').map(String::from).collect();
        for round in &rounds {
            if !valid_game {
                break;
            }
            let plays: Vec<String> = round.split(',').map(String::from).collect();
            for play in &plays {
                let cubes: Vec<&str> = play.split_whitespace().collect();
                let cubes_count: u32 = cubes[0].parse().unwrap();
                let cubes_color = cubes[1];
                if valid_game {
                    if cubes_color == "red" && cubes_count > 12 {
                        valid_game = false;
                        break;
                    } else if cubes_color == "green" && cubes_count > 13 {
                        valid_game = false;
                        break;
                    } else if cubes_color == "blue" && cubes_count > 14 {
                        valid_game = false;
                        break;
                    }
                }
            }
        }

        if valid_game {
            valid_games_sum += i + 1;
        }
    }

    println!("Sum of valid games: {}", valid_games_sum);
}
