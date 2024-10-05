use std::fmt::{Debug, Display};

trait Summary {

    fn get_author(&self) -> &str{
        "Vansh"
    }
    fn summarize(&self) -> String {
        format!("{} Read more...", self.get_author())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}


#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
} 

impl Summary for Tweet {

    fn get_author(&self) -> &str {
        self.username.as_str()
    }

}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} in {}", self.headline, self.author, self.location)
    }

    fn get_author(&self) -> &str {
        self.author.as_str()
    }
}

fn main(


) {

    let tweet1 = Tweet {
        username: String::from("Horse"),
        content: String::from("I am a horse"),
        reply: false,
        retweet: false,
    };

    news_aggregator_trait(&tweet1);

    println!("{:?} ", tweet1);

    let news1 = NewsArticle {
        headline: String::from("Horse"),
        location: String::from("table"),

        author: String::from("Horse"),
        content: String::from("I am a horse"),

    };

    news_aggregator_trait(&news1);

    news_aggregator_trait_generic(&tweet1, &tweet1);

    news_aggregator_trait_generic(&news1, &news1);

    news_aggregator_trait_generic2(&tweet1);

    op(&tweet1, &news1);

    let c = returns_summarizable();

    println!("{}", c.summarize());

    let p = Pair::new(1, 2);

    p.cmp_display();

    let p = Pair::new("a", "b");

    p.cmp_display();
    
}


// fn news_aggregator_tweet(tweet : Tweet) {
//     println!("There is a new news in the town")

//     println!("{} tweeted: {}", tweet.username, tweet.content);

// }

// fn news_aggregator_news(news : NewsArticle) {
//     println!("New news in the town");

//     println!("{} by {} in {}", news.headline, news.author, news.location);
//     println!("Content: {}", news.content);
// }


fn news_aggregator_trait(item: &impl Summary) {
    println!("New news in the town");

   println!("{}", item.summarize());
}

fn news_aggregator_trait_generic<T: Summary>(item: &T, other : &T) {
    println!("New news in the town");

   println!("{}", item.summarize());
    println!("{}", other.summarize());
}

fn news_aggregator_trait_generic2<T: Summary>(item: &T) {
    println!("New news in the town");

   println!("{}", item.summarize());
}

fn op (a: &impl Summary, b: &impl Summary) {
    println!("{}", a.summarize());
    println!("{}", b.summarize());
}

fn op_display<T: Summary + Display>(a: &T, b: &T) {
    println!("{}", a.summarize());
    println!("{}", b.summarize());
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}