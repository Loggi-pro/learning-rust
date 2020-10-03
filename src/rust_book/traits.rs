//define a trait
pub trait Summary {
    //without default behaviour
    //fn summarize(&self) -> String;
    //with default behaviour:
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    fn summarize_author(&self) -> String {
        format!("@Some")
    }
}
//First type implements trait Summary
struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
// Other type implements trait Summary
struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Call summary method for tweet:");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know,people"),
        reply: false,
        retweet: false,
    };
    notify(&tweet);
    //
    println!("Call summary method for NewsArticle");
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    notify(&article);
    //
    println!("Return type which implements a trait from function:");
    notify(&returns_summarizable());
}
//constraints
fn notify<T: Summary>(item: &T) {
    //or fn notify(item: &impl Summary)
    println!("New article available! {}", item.summarize());
}

//return trait
fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
        ),
    }
}
