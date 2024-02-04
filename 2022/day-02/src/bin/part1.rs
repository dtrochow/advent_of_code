use std::fs::read_to_string;

#[derive(Copy, Clone)]
enum Figure {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
    Unknown
}

enum Points {
    Draw = 3,
    Win = 6
}

#[derive(Copy, Clone)]
struct Game {
    my_figure: Figure,
    opponent_figure: Figure
}

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let games = parse_games(lines);
    let score = play_games(games);
    println!("My score: {}", score);
}

fn play_games(games: Vec<Game>) -> u32 {
    let mut score: u32 = 0;
    for game in games {
        score += play_game(game);
    } 
    score
}

fn play_game(game: Game) -> u32 {
    let mut my_score: u32 = game.my_figure as u32;
    match game.my_figure {
        Figure::Rock => match game.opponent_figure {
            Figure::Rock => my_score += Points::Draw as u32,
            Figure::Scissors => my_score += Points::Win as u32,
            _ => ()
        },
        Figure::Paper => match game.opponent_figure {
            Figure::Rock => my_score += Points::Win as u32,
            Figure::Paper => my_score += Points::Draw as u32,
            _ => (),
        },
        Figure::Scissors => match game.opponent_figure {
            Figure::Paper => my_score += Points::Win as u32,
            Figure::Scissors => my_score += Points::Draw as u32,    
            _ => ()
        },
        _ => println!("Wrong figure! -> {}", game.my_figure as u32)
    }
    my_score
}

fn parse_games(lines: Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    for line in lines {
        games.push(parse_game(line));
    }
    games
}

fn parse_game(line: String) -> Game {
    let my_figure = match line.split(' ').last().unwrap() {
        "X" => Figure::Rock,
        "Y" => Figure::Paper,
        "Z" => Figure::Scissors,
         _  => Figure::Unknown
    };
    let opponent_figure = match line.split(' ').next().unwrap() {
        "A" => Figure::Rock,
        "B" => Figure::Paper,
        "C" => Figure::Scissors,
         _  => Figure::Unknown
    };
    Game{ my_figure, opponent_figure }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
