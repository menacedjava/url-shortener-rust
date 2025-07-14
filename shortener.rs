use std::collections::HashMap;

fn main() {
    let mut urls = HashMap::new();
    urls.insert("ggl", "https://google.com");
    urls.insert("gh", "https://github.com");
    println!("gh -> {}", urls["gh"]);
}
