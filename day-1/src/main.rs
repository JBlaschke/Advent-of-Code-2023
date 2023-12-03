use std::fs;
use std::path::{Path, PathBuf};

fn solution_1(input_file:PathBuf) -> i64 {
    let input = fs::read_to_string(input_file).expect(
        "Could not read inputs"
    );

    let mut numbers: Vec<i64> = Vec::new();
    for line in input.split("\n") {
        let mut chars: Vec<char> = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                chars.push(c);
            }
        }
        if chars.len() > 0 {
            let mut valid_digits: Vec<char> = Vec::new();
            valid_digits.push(* chars.first().unwrap());
            valid_digits.push(* chars.last().unwrap());
            let num_string: String = valid_digits.iter().collect();
            numbers.push(num_string.parse().unwrap());
        }
    }

    let mut sum: i64 = 0;
    for n in numbers {
        sum += n;
    }

    return sum;
}


fn main() {
    let problem_1_input: PathBuf = Path::new("data").join("problem_1_input.txt");
    println!("Reading inputs from: {}", problem_1_input.to_string_lossy());

    let answer = solution_1(problem_1_input);
    println!("The answer to problem 1 is: {}", answer);
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn problem_1() {
        assert_eq!(
            crate::solution_1(Path::new("data").join("test_1_input.txt")),
            142
        );
    }
}