pub trait Summary {
    fn summarize( &self ) -> String;
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}


impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!( "Hdl, `{}`; Ath, `{}`; Loc, `{}`", self.headline, self.author, self.location )
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


impl Summary for Tweet {
    fn summarize( &self ) -> String {
        format!( "Usr, `{}`; Cnt, `{}`", self.username, self.content )
    }
}


fn main() {

    let tweet = Tweet {
        username: String::from( "some_user" ),
        content: String::from( "some content"),
        reply: false,
        retweet: false,
    };

    println!( "1 new teet: ``{:?}``", tweet.summarize() );
}
