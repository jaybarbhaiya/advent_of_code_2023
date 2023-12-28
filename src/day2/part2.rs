use core::fmt;

enum Color {
    Red,
    Green,
    Blue,
}

struct Set {
    count: u32,
    color: Color,
}
impl Set {
    fn new(count: u32, color: Color) -> Self {
        Self { count, color }
    }
}

struct Game {
    number: u32,
    sets: Vec<Set>,
}
impl Game {
    fn new(number: u32, sets: Vec<Set>) -> Self {
        Self { number, sets }
    }
}
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = "Game ".to_string();
        out.push_str(&self.number.to_string());
        out.push_str(":");
        for set in &self.sets {
            out.push_str(" ");
            out.push_str(&set.count.to_string());
            out.push_str(" ");
            match set.color {
                Color::Red => {
                    out.push_str("red");
                }
                Color::Green => {
                    out.push_str("green");
                }
                Color::Blue => {
                    out.push_str("blue");
                }
            }
            out.push_str(",");
        }

        write!(f, "{}", out)
    }
}

fn create_game(line: String) -> Option<Game> {
    let mut game: Game;
    let game_data = line.split(':').collect::<Vec<&str>>();
    let game_num = game_data[0].split_whitespace().collect::<Vec<&str>>();
    match game_num[1].parse() {
        Ok(n) => game = Game::new(n, Vec::new()),
        Err(_) => {
            println!("Failed to parse game number");
            return None;
        }
    }
    for set in game_data[1].split(';').collect::<Vec<&str>>() {
        for pair in set.split(',').collect::<Vec<&str>>() {
            let cubes = pair.split_whitespace().collect::<Vec<&str>>();
            match cubes[0].parse::<u32>() {
                Ok(n) => {
                    let c: Color;
                    match cubes[1] {
                        "red" => c = Color::Red,
                        "green" => c = Color::Green,
                        "blue" => c = Color::Blue,
                        _ => {
                            println!("Failed to parse Color: {}", cubes[1]);
                            return None;
                        }
                    }
                    game.sets.push(Set::new(n, c));
                }
                Err(_) => {
                    println!("Failed to parse cubes");
                    return None;
                }
            }
        }
    }
    Some(game)
}

fn return_result(input: String) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        match create_game(line.to_string()) {
            Some(game) => {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;
                for set in game.sets {
                    match set.color {
                        Color::Red => {
                            r = {
                                if set.count > r {
                                    set.count
                                } else {
                                    r
                                }
                            }
                        }
                        Color::Green => {
                            g = {
                                if set.count > g {
                                    set.count
                                } else {
                                    g
                                }
                            }
                        }
                        Color::Blue => {
                            b = {
                                if set.count > b {
                                    set.count
                                } else {
                                    b
                                }
                            }
                        }
                    }
                }
                sum += r * g * b;
            }
            None => println!("Failed to create game from {}", line),
        }
    }
    sum
}

pub fn part2() {
    let input = include_str!("./game_input.txt");
    let output = return_result(input.to_string());
    println!("Day2 part2 result: {}", output);
}
