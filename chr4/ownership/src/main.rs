fn main() {
    let s1 = gives_ownership();
    println!("s1 is {s1}");

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    let (s4, len) = calculate_length(s1);
    println!("length of {s4} is {len}");

    let len = calculate_length_ref(&s4);
    println!("The length of '{}' is {}.", s4, len);

    let mut s5 = String::from("hei");
    change(&mut s5);

    let mut s = String::from("Mutable");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");

    let r3 = &mut s;
    println!("{r3}");

    // slices
    let mut s = String::from("Hello world");
    let word = first_word(&s);
    s.clear();
    println!("first word from {s} is {word}");
    // The solution for finding words inside strings
    // is called 'string slice'
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("Frist word is {hello}, second {world} and string is {s}");
    // first word using slices
    let s = String::from("Hello world");
    let word = first_word_slice(&s);
    println!("first word in '{s}' is '{word}'");

    // Other string slice example
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &items) in bytes.iter().enumerate() {
        if items == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_slice_improve(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if (item == b' ') {
            return &s[0..i];
        }
    }
    &s[..]
}
