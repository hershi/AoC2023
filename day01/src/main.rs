use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn translate(s: &str) -> String {
    let digits = vec![ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", ];
    let mut result = String::new();

    let mut i = 0;
    while i < s.len() {
        for d in 0..digits.len() {
            if s[i..].starts_with(digits[d]) {
                result.push(char::from_digit(d as u32+1, 10).unwrap());
                break;
            }
        }

        result.push(s.chars().nth(i).unwrap());
        i+=1;
    }

    result
}

fn read_input() -> Vec<String> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x| x.unwrap())
        .collect()
}

fn part_1(lines: &Vec<String>) {
    let sum = lines.iter()
        .map(|l| l.chars().filter(|c| c.is_ascii_digit()).map(|c|c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .map(|ds| ds[0] * 10 + ds[ds.len()-1])
        .sum::<u32>();

    println!("{:?}", sum);
}

fn part_2(lines: &Vec<String>) {
    part_1(&lines.iter()
           .map(|l| translate(l)).collect());
}

fn main() {
    let input = read_input();
    part_1(&input);
    println!("Part 1 Done!");
    part_2(&input);
    println!("Done!");
}
