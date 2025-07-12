// use aggregator::{NewsArticle, SocialPost, Summary};
use aggregator::{SocialPost, Summary};

// pub fn notify(item: &impl Summary) {
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

fn some_function_where<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

fn returns_summarizable(item: &impl Summary) -> impl Summary {
    SocialPost {
        username: String::from("example_user"),
        content: String::from("This is an example content."),
        reply: false,
        repost: false,
    }
}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new social post: {}", post.summarize());

    // let article = NewsArticle {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from(
    //         "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
    //     ),
    // };

    // println!("1 new article: {}", article.summarize());

    notify(&post);
}
