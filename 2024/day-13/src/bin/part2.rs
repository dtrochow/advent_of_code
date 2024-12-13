use aoc_lib::read_lines;
use fraction::{BigFraction, Zero};

type BF = BigFraction;

#[derive(Debug, Clone)]
struct Equation {
    /* a * x + b * y = c */
    a: BF,
    b: BF,
    c: BF,
}

type EquationsPair = (Equation, Equation);

fn solve_equation(equ_1: Equation, equ_2: Equation) -> Option<(BF, BF)> {
    /* Cramer's rule */
    let a = ((equ_1.c.clone() * equ_2.b.clone()) - (equ_1.b.clone() * equ_2.c.clone()))
        / ((equ_1.a.clone() * equ_2.b.clone()) - (equ_1.b.clone() * equ_2.a.clone()));
    let b = ((equ_1.a.clone() * equ_2.c.clone()) - (equ_1.c.clone() * equ_2.a.clone()))
        / ((equ_1.a.clone() * equ_2.b.clone()) - (equ_1.b.clone() * equ_2.a.clone()));
    if a.fract() == BigFraction::zero() && b.fract() == BigFraction::zero() {
        Some((a, b))
    } else {
        None
    }
}

fn get_equations_pairs(lines: Vec<String>) -> Vec<EquationsPair> {
    let mut equations_pairs: Vec<EquationsPair> = Vec::new();
    let (mut a1, mut b1, mut a2, mut b2) = (0.0, 0.0, 0.0, 0.0);
    let (mut c1, mut c2): (f64, f64);
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
                        a: BigFraction::from(a1),
                        b: BigFraction::from(b1),
                        c: BigFraction::from(c1 + 10000000000000.0),
                    },
                    Equation {
                        a: BigFraction::from(a2),
                        b: BigFraction::from(b2),
                        c: BigFraction::from(c2 + 10000000000000.0),
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

    let mut tokens_count: u64 = 0;
    for pair in equations_pairs {
        let solution = solve_equation(pair.0, pair.1);
        if let Some(eq_solution) = solution {
            let a: u64 = eq_solution.0.try_into().unwrap();
            let b: u64 = eq_solution.1.try_into().unwrap();
            tokens_count += a * 3 + b;
        }
    }
    println!("Tokens count: {}", tokens_count);
}
