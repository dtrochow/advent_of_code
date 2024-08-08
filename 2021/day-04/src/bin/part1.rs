use std::fs::read_to_string;

type BingoCard = Vec<Vec<u32>>;

const BINGO_CARD_SIZE: usize = 5;

fn main() {
    let lines: Vec<String> = read_lines("./src/bin/input1.txt");
    let bingo_numbers: Vec<u32> = get_bingo_numbers(lines.first().unwrap());
    let bingo_cards: Vec<BingoCard> = get_bingo_cards(&lines[2..]);
    println!("Number of cards: {}", bingo_cards.len());

    let mut game_numbers: Vec<u32> = Vec::new();
    for number in bingo_numbers {
        game_numbers.push(number);
        for bingo_card in &bingo_cards {
            if is_win_card(bingo_card, &game_numbers) {
                let score: u64 = calculate_winning_score(bingo_card, &game_numbers);
                println!("Bingo! Score: {}", score);
                return;
            }
        }
    }
}

fn calculate_winning_score(card: &BingoCard, game_numbers: &[u32]) -> u64 {
    let current_number: u64 = (*game_numbers.last().unwrap()).into();
    let mut sum: u64 = 0;
    let mut marked_sum: u64 = 0;
    for row in card {
        row.iter().for_each(|num| {
            if game_numbers.contains(num) {
                marked_sum += *num as u64;
            }
        });
        sum += row.iter().sum::<u32>() as u64;
    }
    sum -= marked_sum;

    sum * current_number
}

fn transpose(matrix: &[Vec<u32>]) -> Vec<Vec<u32>> {
    if matrix.is_empty() {
        return vec![];
    }
    (0..matrix[0].len())
        .map(|i| matrix.iter().map(|row| row[i]).collect())
        .collect()
}

fn is_win_card(card: &BingoCard, numbers: &[u32]) -> bool {
    for row in card {
        if row.iter().all(|num| numbers.contains(num)) {
            return true;
        }
    }
    let transposed = transpose(card);
    for row in transposed {
        if row.iter().all(|num| numbers.contains(num)) {
            return true;
        }
    }

    false
}

fn get_bingo_numbers(line: &str) -> Vec<u32> {
    line.split(',').map(|x| x.parse::<u32>().unwrap()).collect()
}

fn get_bingo_cards(lines: &[String]) -> Vec<BingoCard> {
    let mut bingo_cards: Vec<BingoCard> = Vec::<BingoCard>::new();

    let mut bingo_card: BingoCard = BingoCard::new();
    for line in lines {
        if line.is_empty() {
            continue;
        };

        let card_row = line
            .trim_start()
            .replace("  ", " ")
            .split(' ')
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect();
        bingo_card.push(card_row);

        if bingo_card.len() >= BINGO_CARD_SIZE {
            bingo_cards.push(bingo_card.clone());
            bingo_card.clear();
        }
    }

    bingo_cards
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
