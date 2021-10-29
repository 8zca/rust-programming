// トレイトはインタフェースに類似している
pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Display {
    fn display(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// NewsArticle implements Summary的な処理
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Summaryトレイトを実装しているものならOK
// fn notify<T: Summary>(item: &T) { ... } でもいい
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// SummaryとDisplayを実装しているならOK
fn notify2(item: &(impl Summary + Display)) {}
fn notify3<T: Summary + Display>(item: &T) {}

// where句を使っても書ける(複雑な型の組み合わせはこっち)
fn notify4<T>(t: &T)
    where T: Summary + Display,
{}


fn main() {
    let article = NewsArticle {
        headline: String::from("title"),
        location: String::from("tokyo"),
        author: String::from("taro"),
        content: String::from("abcabc")
    };
    println!("{}", article.summarize());
    notify(&article);
}
