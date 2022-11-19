fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();
    let s = "initial contents".to_string();

    // updating strings
    let mut s = String::from("Hi");
    s.push_str(" man");
    println!("{s}");

    // string after appending
    let mut s1 = String::from("Hello ");
    let s2 = "world";
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");

    // Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    // String formating
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{s1} {s2} {s3}");
    println!("{s}");

    for c in "hello".chars() {
        print!("{c}");
    }
    println!();
    for b in "hello".bytes() {
        print!("{b} ");
    }
    println!();
}
