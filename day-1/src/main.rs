use std::fs;
use std::path::{Path, PathBuf};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref TOKENS: Vec<&'static str> = {
        let tokens_1 = [
                "zero", "one", "two", "three", "four", "five", "six", "seven",
                "eight", "nine"
            ];
        let mut tokens_2: Vec<&str> = Vec::new();
        for t in tokens_1 {
            tokens_2.push(t);
        }
        tokens_2
    };
    static ref OFFSETS: Vec<usize> = {
        // There could be overlaps between tokens => remove max possible overlap
        // distance from the offsets
        let token_overlaps = [1, 1, 1, 1, 0, 1, 0, 1, 1, 1];
        let mut offsets: Vec<usize> =  Vec::new();
        for (i, t) in TOKENS.iter().enumerate() {
            offsets.push(t.len() - token_overlaps[i]);
        }
        offsets
    };
}

fn match_tokens(string: &str) -> Vec<char> {
    let mut number_chars: Vec<char> = Vec::new();

    let mut char_iter = string.chars().enumerate().into_iter();
    while let Some((i, c)) = char_iter.next() {
        if c.is_numeric() {
            number_chars.push(c);
            continue;
        }

        for (ti, t) in TOKENS.iter().enumerate() {
            if string[i..].starts_with(t) {
                number_chars.push(char::from_digit(ti as u32, 10).unwrap());
                // Skip ahead by the length of the matched token
                char_iter.by_ref().take(OFFSETS[ti] - 1).for_each(drop);
                break;
            }
        }
    }
 
    return number_chars;
}


fn solution_1(input_file: & PathBuf) -> i64 {
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


fn solution_2(input_file: & PathBuf) -> i64 {
    let input = fs::read_to_string(input_file).expect(
        "Could not read inputs"
    );

    let mut numbers: Vec<i64> = Vec::new();
    for line in input.split("\n") {
        let chars = match_tokens(line);

        if chars.len() > 0 {
            let mut valid_digits: Vec<char> = Vec::new();
            valid_digits.push(* chars.first().unwrap());
            valid_digits.push(* chars.last().unwrap());
            let num_string: String = valid_digits.iter().collect();
            numbers.push(num_string.parse().unwrap());
        }else {
            println!("{}", line);
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

    let answer_1 = solution_1(& problem_1_input);
    println!("The answer to problem 1 is: {}", answer_1);

    let answer_2 = solution_2(& problem_1_input);
    println!("The answer to problem 2 is: {}", answer_2);
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn problem_1() {
        assert_eq!(
            crate::solution_1(& Path::new("data").join("test_1_input.txt")),
            142
        );
    }
    #[test]
    fn problem_2() {
        assert_eq!(
            crate::solution_2(& Path::new("data").join("test_2_input.txt")),
            281
        );
    }
}