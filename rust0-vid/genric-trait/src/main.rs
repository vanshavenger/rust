trait  Summary {
    fn summarize(&self) -> String{
        String::from("(Read more...)")
    }
}

trait Display {
    fn display(&self) -> String{
        String::from("Displaying...")
    }
}

struct NewsArticle {
    headline: String,
    location: String,

}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, in {}", self.headline, self.location)
    }
}

impl Display for NewsArticle {
    fn display(&self) -> String {
        format!("{}, by {}", self.headline, self.location)
    }
}



fn notify (u : &impl Summary) {
    println!("Breaking news! {}", u.summarize());
}

fn multi_trait<T: Summary + Display>(u: &T) {
    println!("Breaking news! {}", u.summarize());
    println!("Breaking news! {}", u.display());
}

fn main() {

    // generic function
    let cmp_number = bigger_kon(3,2);
    let cmp_chat = bigger_kon('a','b');

    println!("The bigger number is: {}", cmp_number);
    println!("The bigger char is: {}", cmp_chat);

    println!("-----------------------------");

    let article = NewsArticle {
        headline: String::from("Vansh Chopra"),
        location: String::from("SF"),
    };

    println!("{}", article.summarize() );

    // notify(article);
    notify(&article);

    println!("-----------------------------");


    multi_trait(&article);
}

fn bigger_kon<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
