use std::collections::HashMap;

#[derive(Debug)]

enum Median {
    Single(Option<u8>),
    Double(Option<(u8, u8)>),
}

fn median_sorted(numbers: &Vec<u8>) -> Median {
    match numbers.len() % 2 {
        0 => Median::Double(Option::Some((
            numbers[numbers.len() / 2],
            numbers[numbers.len() / 2 - 1],
        ))),
        1 => Median::Single(Option::Some(numbers[numbers.len() / 2])),
        _ => Median::Single(Option::None),
    }
}

fn most_occurences(numbers: Vec<u8>) -> u8 {
    let mut occurences: HashMap<u8, u8> = HashMap::new();
    for number in numbers {
        *occurences.entry(number).or_insert(0) += 1;
    }
    let most_occurences = occurences.iter().max_by_key(|entry| entry.1).unwrap();
    *most_occurences.1
}

fn main() {
    let mut numbers: Vec<u8> = Vec::with_capacity(11);
    for _ in 0..numbers.capacity() {
        numbers.push(rand::random());
    }
    numbers.sort_unstable();

    for number in &numbers {
        print!("{number} ");
    }
    println!();
    let median: Median = median_sorted(&numbers);
    println!("Median is {:?}", median);

    let most_occurences = most_occurences(numbers);
    println!("most numbers occurences: {most_occurences}");
}
