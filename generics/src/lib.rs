pub mod aggregator {

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    pub trait Summary {
        fn summarize(&self);
    }
    
    impl Summary for NewsArticle {
        fn summarize(&self) {
            println!("headline {} by {} ({})", self.headline, self.author, self.location);
        }
    }
    
    impl Summary for Tweet {
        fn summarize(&self) {
            println!("tweet {}: {}", self.username, self.content);
        }
    }
   
}