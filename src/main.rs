use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

#[derive(Debug)]
struct Rucksack {
    compartment: Vec<String>,
}

#[derive(Debug)]
enum ParseRucksackError {
    NotDivisibleByTwo,
}

impl FromStr for Rucksack {
    type Err = ParseRucksackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.len() % 2 != 0 {
            return Err(ParseRucksackError::NotDivisibleByTwo);
        }

        let (left, right) = s.split_at(s.len() / 2);

        Ok(Rucksack {
            compartment: vec![left.to_string(), right.to_string()],
        })
    }
}

fn get_common_chars(input: &[String]) -> Vec<char> {
    let mut map = HashMap::new();
    for (i, str) in input.iter().enumerate() {
        for c in str.chars() {
            let set = map.entry(c).or_insert_with(HashSet::new);
            set.insert(i);
        }
    }

    map.into_iter()
        .filter(|(_, v)| v.len() == input.len())
        .map(|(k, _)| k)
        .collect()
}

fn convert_char_to_int(c: char) -> Option<i32> {
    match c {
        'a'..='z' => Some(c as i32 - b'a' as i32 + 1),
        'A'..='Z' => Some(c as i32 - b'A' as i32 + 27),
        _ => None,
    }
}

fn main() {
    let file_path = "input_3.txt";

    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    let rucksacks: Vec<Rucksack> = contents
        .lines()
        .map( |line| line.parse().unwrap())
        .collect();

    let sum: i32 = rucksacks
        .iter()
        .map(|x| get_common_chars(&x.compartment))
        .map(|x| convert_char_to_int(x[0]).unwrap())
        .sum();

    println!("The sum is: {sum}");

}
