use traits::{SosPost, Summary};

fn main() {
    let post = SosPost {
        uname: "RockWheelLiquid".to_string(),
        cont: String::from("of course, as you probably already know, people"),
        reply: false,
        rpost: false,
    };
    println!("1 new post: {}", post.summarize());
}
