use chr17_1::AveragedCollection;

fn main() {
    let mut avg_col = AveragedCollection::default();
    println!("Average is {}", avg_col.average());
    avg_col.add(1);
    avg_col.add(2);
    avg_col.add(3);
    println!("Average is {}", avg_col.average());
}
