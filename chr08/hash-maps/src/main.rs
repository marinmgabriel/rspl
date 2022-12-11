use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Replacing a value from a specific key
    println!("{:?}", scores);
    scores.insert(String::from("Blue"), 30);
    println!("{:?}", scores);

    // Check for a certain key
    scores.entry(String::from("Black")).or_insert(100);
    scores.entry(String::from("Yellow")).or_insert(70);
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key} {value}");
    }

    // Hash map takes the ownership of strings
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // count word occurences
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
