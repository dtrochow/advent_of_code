use std::fs::read_to_string;

struct Set {
    red: u32,
    green: u32,
    blue: u32
}

enum Color {
    Red(u32),
    Green(u32),
    Blue(u32)
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let mut sum: u32 = 0;
    for line in lines {
        sum += get_minimum_cubes_power(line);
    }
    println!("The sum is: {}", sum);
}

fn parse_cube_num(cube_desc: String) -> Option<Color> {
    let cube_key_val: Vec<&str> = cube_desc.trim_start().split(' ').collect();

    let color_string = *cube_key_val.last().unwrap();
    let cubes_quantity: u32 = cube_key_val.first().unwrap().parse().unwrap();

    match color_string {
        "red" => return Some(Color::Red(cubes_quantity)),
        "green" => return Some(Color::Green(cubes_quantity)),
        "blue" => return Some(Color::Blue(cubes_quantity)),
        &_ => None
    }
}

fn parse_game_set(set_desc: String) -> Set {
    let mut game = Set{red:0, green:0, blue:0};
    let mut cubes_with_values: Vec<Color> = Vec::<Color>::new();

    let cubes: Vec<&str> = set_desc.split(',').collect();
    for cube in cubes {
        match parse_cube_num(cube.to_string()) {
            Some(value) => cubes_with_values.push(value),
            None => println!("Failure during games set processing"),
        }
    }
    for cube_val in cubes_with_values {
        match cube_val {
            Color::Red(num) => game.red = num,
            Color::Green(num) => game.green = num,
            Color::Blue(num) => game.blue = num,
        }
    }
    game
}

fn get_each_set_descriptors(game_desc: String) -> Vec<Set> {
    let game: String = game_desc.split(':').last().unwrap_or_default().to_string();
    let game_sets: Vec<&str> = game.split(';').collect();

    let mut sets_vec: Vec<Set> = Vec::<Set>::new();
    for set in game_sets {
        sets_vec.push(parse_game_set(set.to_string()));
    }

    sets_vec
}

fn get_minimum_cubes_power(game_desc: String) -> u32 {
    let sets: Vec<Set> = get_each_set_descriptors(game_desc);
    let mut max_red = 0;
    let mut max_blue = 0;
    let mut max_green = 0;

    for set in sets {
        if set.red > max_red { max_red = set.red }
        if set.green > max_green { max_green = set.green }
        if set.blue > max_blue { max_blue = set.blue }
    }
    
    max_red * max_blue * max_green
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
