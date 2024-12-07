use aoc_lib::read_lines;

#[derive(Debug)]
struct Test {
    value: u64,
    test: Vec<u64>,
}

fn get_tests(lines: Vec<String>) -> Vec<Test> {
    let mut tests: Vec<Test> = Vec::new();
    for line in lines {
        tests.push(Test {
            value: line.split(':').next().unwrap().parse().unwrap(),
            test: line
                .split(':')
                .last()
                .unwrap()
                .split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect(),
        })
    }
    tests
}

fn number_to_bits(num: u64, bits_num: usize) -> Vec<bool> {
    (0..bits_num).map(|i| (num >> i) & 1 != 0).collect()
}

fn is_test_valid(test: &Test) -> bool {
    let operations_count = test.test.len() - 1;
    let operation_permutations_count = 1 << operations_count;
    for permutation in 1..operation_permutations_count + 1 {
        let operations = number_to_bits(permutation, operations_count);
        let mut value: u64 = *test.test.first().unwrap();
        for (i, operation) in operations.iter().enumerate() {
            if *operation {
                value += test.test[i + 1];
            } else {
                value *= test.test[i + 1];
            }
            if value > test.value {
                break;
            }
        }
        if value == test.value {
            return true;
        }
    }
    false
}

fn get_valid_test_indexes(tests: &[Test]) -> Vec<usize> {
    let mut valid_test_indexes: Vec<usize> = Vec::new();
    for (index, test) in tests.iter().enumerate() {
        if is_test_valid(test) {
            valid_test_indexes.push(index);
        }
    }
    valid_test_indexes
}

fn sum_all_valid_test_values(tests: Vec<Test>, valid_indexes: Vec<usize>) -> u64 {
    let mut sum = 0;
    for index in valid_indexes {
        sum += tests[index].value;
    }
    sum
}

fn main() {
    let lines = read_lines("./src/bin/input1.txt");

    let tests = get_tests(lines);
    let valid_test_indexes = get_valid_test_indexes(&tests);

    let sum_of_valid_test_values = sum_all_valid_test_values(tests, valid_test_indexes);
    println!("Sum of valid test values: {}", sum_of_valid_test_values);
}
