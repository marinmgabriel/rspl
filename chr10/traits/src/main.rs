use traits::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("someone"),
        content: String::from("Moving to mastodon"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize_author());
    println!("Summarized tweet:\n {}", tweet.summarize());
}
