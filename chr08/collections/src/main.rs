fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        print!("{} ", i);
    }
    println!();

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        print!("{} ", i);
    }
    println!();

    // Vector with different types
    enum SpreadsheedCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheedCell::Int(3),
        SpreadsheedCell::Text(String::from("blue")),
        SpreadsheedCell::Float(10.12),
    ];
}
