use std::fs;
use std::ops::AddAssign;

enum Colour {
    Red,
    Green,
    Blue,
}

struct CubeTotals {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeTotals {
    fn new() -> Self {
        CubeTotals {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

struct Game {
    id: u32,
    totals: Vec<CubeTotals>,
}

impl Game {
    fn is_valid(&self, max_cubes: &CubeTotals) -> bool {
        for total in &self.totals {
            if max_cubes.red < total.red
                || max_cubes.blue < total.blue
                || max_cubes.green < total.green
            {
                return false;
            }
        }
        true
    }
}

impl TryFrom<&str> for Game {
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        //Example : Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        dbg!(value);
        value
            .split_once(':')
            .and_then(|(game_str, totals_str)| {
                if let Some(Ok(id)) = game_str
                    .split_once(' ')
                    .map(|(_, id_str)| id_str.parse::<u32>())
                {
                    let totals = totals_str
                        .split(';')
                        .map(|colors| {
                            colors
                                .split(',')
                                .filter_map(|color| {
                                    let mut color = color.split_whitespace();
                                    let number = color.next().map(|num| num.parse::<u32>());
                                    let color = color.next().map(|color| match color {
                                        "red" => Colour::Red,
                                        "green" => Colour::Green,
                                        "blue" => Colour::Blue,
                                        _ => panic!(""),
                                    });
                                    match (number, color) {
                                        (Some(Ok(n)), Some(c)) => Some((n, c)),
                                        _ => None,
                                    }
                                })
                                .fold(CubeTotals::new(), |mut c_t, (num, colour)| {
                                    match colour {
                                        Colour::Red => c_t.red += num,
                                        Colour::Green => c_t.green += num,
                                        Colour::Blue => c_t.blue += num,
                                    };
                                    c_t
                                })
                        })
                        .collect();
                    return Some(Game { id, totals });
                }
                None
            })
            .ok_or("Did not get the game".into())
    }

    type Error = String;
}

fn process_input(max_cubes: CubeTotals, str_input: &String) -> u32 {
    let valid_games = str_input
        .lines()
        .flat_map(Game::try_from)
        .filter(|games| games.is_valid(&max_cubes));

    valid_games.fold(0, |mut i, g| {
        i += g.id;
        i
    })
}

fn main() {
    fs::read_to_string("input.txt").map(|file| {
        let res = process_input(
            CubeTotals {
                red: 12,
                green: 13,
                blue: 14,
            },
            &file,
        );
        println!("{}", res);
    }).expect("No File Found");
}

#[test]
fn test_1() {
    let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;
    assert_eq!(
        process_input(
            CubeTotals {
                red: 12,
                green: 13,
                blue: 14
            },
            &input.to_string()
        ),
        8
    );
}
