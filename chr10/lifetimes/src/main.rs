use std::fmt::Display;

fn main() {
    {
        let string1 = String::from("long string is long");
        let result;
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    {
        let string1 = String::from("looong");
        let string2 = String::from("short");
        let announcement = String::from("The longest string is...");
        let result = longest_with_announcement(string1.as_str(), string2.as_str(), announcement);
        println!("{result}");
    }

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Coult not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("Extracted part: {}", i.part);
    }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
