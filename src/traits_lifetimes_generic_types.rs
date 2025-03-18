#![allow(dead_code)]
use std::fmt::Display;

pub fn main() {
    println!("Generic Data Types, Traits And Lifetimes!");
    generic_data_types();
    traits();
    lifetimes();
}

fn generic_data_types() {
    struct Point<T, U, X> {
        x: T,
        y: U,
        z: X,
    }

    let x: Point<i8, String, f32> = Point {
        x: 1,
        y: String::from("Omg"),
        z: 2.54,
    };

    println!("Point: x = {}, y = {}, z = {}", x.x, x.y, x.z);

    impl<T, U, X> Point<T, U, X> {
        fn x_x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32, f32, f32> {
        fn distance_from_origin(&self) -> f32 {
            (&self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    println!("x = {}", x.x_x());

    let y: Point<f32, f32, f32> = Point {
        x: 4.2,
        y: 6.9,
        z: 4.20,
    };

    println!("y = {}", y.distance_from_origin());

    impl<X1, Y1, Z1> Point<X1, Y1, Z1> {
        fn mix_up<X2, Y2, Z2>(&self, other: Point<X2, Y2, Z2>) -> Point<&X1, Y2, Z2> {
            Point {
                x: &self.x,
                y: other.y,
                z: other.z,
            }
        }
    }

    let p1 = Point {
        x: 5,
        y: 10.4,
        z: &x.z,
    };
    let p2 = Point {
        x: "Hello",
        y: 'c',
        z: p1.z,
    };

    let p3 = p1.mix_up(p2);
    println!("p3.x = {}, p3.y = {}, p3.z = {}", p3.x, p3.y, p3.z);
}

fn traits() {
    trait Summary {
        fn summarize(&self) -> String;
    }

    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    struct Person {
        name: String,
        age: u8,
    }

    let tweet: Tweet = Tweet {
        username: String::from("King"),
        content: String::from("I Am Pro"),
        reply: true,
        retweet: false,
    };

    println!("Summary: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceberg"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    impl Summary for Person {
        fn summarize(&self) -> String {
            format!("{}: {}", self.name, self.age)
        }
    }

    fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    let person: Person = Person {
        name: String::from("King"),
        age: 69,
    };

    notify(&tweet);

    notify(&article);

    notify(&person);

    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
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

    let x = Pair::new(5, 10);

    x.cmp_display();
}

fn lifetimes() {
    let x = 5;

    let r = &x;

    println!("r: {r} and x: {x}");

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else if x.len() < y.len() {
            y
        } else {
            "Both Are Equal."
        }
    }

    let x = "Pro";
    let y = "Ooga Booga";

    println!("x: {}, y: {}, Longest: {}", x, y, longest(&x, &y));

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {ann}");
        if x.len() > y.len() { x } else { y }
    }
}
