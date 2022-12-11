use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();
    //pig_latin(&mut input);
    let mut result = pig_latin_sentence(&mut input);
    println!("{}", input);
    println!("{}", result);
    input.clear();
    result.clear();
}

fn pig_latin_sentence(input: &str) -> String {
    let mut result = String::new();

    for mut word in input.split_whitespace() {
        match word.chars().all(char::is_alphanumeric) {
            true => {
                let mut word_string = String::new();
                word_string = word.trim().to_string();
                pig_latin(&mut word_string);
                result.push_str(word_string.as_mut_str());
                result.push_str(" ");
            }
            false => {
                result.push_str(word);
                result.push_str(" ");
            }
        }
    }

    result
}

fn pig_latin(input: &mut String) -> &mut str {
    let initial_letter = input.remove(0);
    if !is_vowel(initial_letter) {
        input.push(initial_letter);
        input.push_str("-ay");
    } else {
        input.push_str("-hay");
    }
    input.as_mut_str()
}

fn is_vowel(c: char) -> bool {
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let vowels_up: [char; 5] = ['A', 'E', 'I', 'O', 'U'];
    vowels.contains(&c) || vowels_up.contains(&c)
}
