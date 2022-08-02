fn main() {
    generics();
    traits();
}

fn generics() {
    fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 10, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 't', 'b', 'y', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    struct Point<T, U> {
        x: T,
        y: U,
    }

    let _both_integer = Point { x: 5, y: 10 };
    let _both_float = Point { x: 1.0, y: 4.0 };
    let _integer_and_float = Point { x: 5, y: 4.0 };
}

fn traits() {
    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            String::from(format!("Read more from {}...", self.summarize_author()))
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
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

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
