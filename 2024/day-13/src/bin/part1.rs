use aoc_lib::read_lines;

#[derive(Debug)]
struct Equation {
    /* a * x + b * y = c */
    a: f64,
    b: f64,
    c: f64,
}

type EquationsPair = (Equation, Equation);

fn solve_equation(equ_1: Equation, equ_2: Equation) -> Option<(f64, f64)> {
    /* Cramer's rule */
    let a =
        ((equ_1.c * equ_2.b) - (equ_1.b * equ_2.c)) / ((equ_1.a * equ_2.b) - (equ_1.b * equ_2.a));
    let b =
        ((equ_1.a * equ_2.c) - (equ_1.c * equ_2.a)) / ((equ_1.a * equ_2.b) - (equ_1.b * equ_2.a));
    if a.fract() == 0.0 && b.fract() == 0.0 {
        Some((a, b))
    } else {
        None
    }
}

fn get_equations_pairs(lines: Vec<String>) -> Vec<EquationsPair> {
    let mut equations_pairs: Vec<EquationsPair> = Vec::new();
    let (mut a1, mut b1, mut a2, mut b2) = (0.0, 0.0, 0.0, 0.0);
    let (mut c1, mut c2);
    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }
        let entry_row = index % 4;
        match entry_row {
            0 => {
                a1 = line
                    .split("X+")
                    .last()
                    .unwrap()
                    .split(',')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                a2 = line.split("Y+").last().unwrap().parse().unwrap();
            }
            1 => {
                b1 = line
                    .split("X+")
                    .last()
                    .unwrap()
                    .split(',')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                b2 = line.split("Y+").last().unwrap().parse().unwrap();
            }
            2 => {
                c1 = line
                    .split("X=")
                    .last()
                    .unwrap()
                    .split(',')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                c2 = line.split("Y=").last().unwrap().parse().unwrap();

                equations_pairs.push((
                    Equation {
                        a: a1,
                        b: b1,
                        c: c1,
                    },
                    Equation {
                        a: a2,
                        b: b2,
                        c: c2,
                    },
                ));
            }
            _ => (),
        }
    }
    equations_pairs
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");
    let equations_pairs = get_equations_pairs(lines);

    let mut tokens_count = 0;
    for pair in equations_pairs {
        let solution = solve_equation(pair.0, pair.1);
        if let Some(eq_solution) = solution {
            tokens_count += eq_solution.0 as i32 * 3 + eq_solution.1 as i32;
        }
    }
    println!("Tokens count: {}", tokens_count);
}
