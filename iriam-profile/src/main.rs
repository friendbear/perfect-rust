//! This module contains the main code for the Iriam profile application.
//!
//! The code defines the `LiveStreamer` struct, which represents a live streamer profile.
//! It also includes a builder pattern implementation for creating instances of `LiveStreamer`.
//! The `Builder` struct provides methods for setting the name, mark, and handle names of the streamer.
//! The `LiveStreamer` struct implements the `Display` trait for custom formatting.
//! The main function creates a list of streamers using the builder pattern and prints their details.
//!
//! # Example
//!
//! ```rust
//! use iriam_profile::Builder;
//!
//! let streamer = Builder::new()
//!     .with_name("MyStreamer")
//!     .with_mark("12345")
//!     .with_handle_names(vec!["handle1", "handle2"])
//!     .build();
//!
//! println!("{}", streamer);
//! ```
//!
//! For more information, see the documentation of the individual types and methods.
use std::fmt::Display;

type S = LiveStreamer;
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum LiveStreamerPlatform {
    Youtube,
    Iriam,
    Etc,
}
#[derive(Debug, Default, Clone)]
struct LiveStreamer {
    name: Option<String>,
    mark: Option<String>,
    handle_names: Option<Vec<String>>,
    // platforms: Option<Vec<LiveStreamerPlatform>>,
}
impl Display for LiveStreamer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}\n{:?}",
            self.name.as_deref().unwrap_or_default(),
            self.mark.as_deref().unwrap_or_default(),
            self.handle_names.as_deref().unwrap_or_default().join(", ")
        )
    }
}

struct Builder(LiveStreamer);
fn main() {
    let streamer = vec![
        Builder::new()
            .with_name("eL(神様)&ミタマ")
            .with_handle_names(vec!["@mitama_sama"])
            .build(),
        Builder::new()
            .with_name("逢坂きゅうり。")
            .with_handle_names(vec!["@aisakakyuuuuuri"])
            .build(),
        Builder::new()
            .with_name("晴陽かりん")
            .with_mark("🐱💗🐟")
            .with_handle_names(vec!["@harenohi_kaarin"])
            .build(),
        Builder::new()
            .with_name("神園える")
            .with_mark("")
            .with_handle_names(vec!["@OlangeEru", "@eru_purple"])
            .build(),
        Builder::new()
            .with_name("ｷｮﾑﾉｶﾀﾏﾘ")
            .with_mark("")
            .with_handle_names(vec!["@kyomu_sofione"])
            .build(),
        Builder::new()
            .with_name("天傘ぱらぱ")
            .with_mark("🌩️💛")
            .with_handle_names(vec!["@AmagasaPalapa"])
            .build(),
        Builder::new()
            .with_name("花ノ木もえ")
            .with_mark("☁️🎀")
            .build(),
        Builder::new().with_mark("📘📗🌼").build(),
        Builder::new().with_mark("🐈‍⬛💜.*･").build(),
        Builder::new()
            .with_name("はしちゃん")
            .with_mark("🥢💙🖤")
            .with_handle_names(vec!["@hash_iriam", "@babu_hashi"])
            .build(),
    ];
    let printer = |s: S| {
        println!(
            "♡{}, {}, {}",
            s.name.as_deref().unwrap_or_default(),
            s.mark.as_deref().unwrap_or_default(),
            s.handle_names.as_deref().unwrap_or_default().join(",")
        )
    };
    streamer.iter().enumerate().for_each(|s| {
        println!("{}, {}", s.0, s.1);
    });
    streamer.into_iter().for_each(printer);

}
/// Builder struct for creating instances of LiveStreamer.
///
/// The Builder struct provides a fluent interface for configuring and building instances of LiveStreamer.
/// It allows setting the name, mark, and handle names of the LiveStreamer.
/// Once all the desired properties are set, the `build` method is used to create the final LiveStreamer instance.
///
/// # Example
///
/// ```
/// let streamer = Builder::new()
///     .with_name("MyStreamer")
///     .with_mark("12345")
///     .with_handle_names(vec!["handle1", "handle2"])
///     .build();
/// ```
impl Builder {
    /// Creates a new instance of the Builder.
    ///
    /// # Example
    ///
    /// ```
    /// let builder = Builder::new();
    /// ```
    fn new() -> Self {
        Builder(LiveStreamer::default())
    }

    /// Sets the name of the LiveStreamer.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the LiveStreamer.
    ///
    /// # Returns
    ///
    /// The updated Builder instance.
    ///
    /// # Example
    ///
    /// ```
    /// let builder = Builder::new().with_name("MyStreamer");
    /// ```
    fn with_name(&self, name: &str) -> Self {
        Builder(LiveStreamer {
            name: Some(name.to_string().clone()),
            ..self.0.clone()
        })
    }
    fn with_mark(&self, mark: &str) -> Self {
        Builder(LiveStreamer {
            mark: Some(mark.to_string().clone()),
            ..self.0.clone()
        })
    }
    fn with_handle_names(&self, handle_names: Vec<&str>) -> Self {
        Builder(LiveStreamer {
            handle_names: Some(handle_names.iter().map(|str| str.to_string()).collect()),
            ..self.0.clone()
        })
    }
    fn build(self) -> LiveStreamer {
        self.0
    }
}

#[allow(dead_code)]
fn str_sort() {
    let mut s: Vec<char> =
        "🥢💙🖤.｡.:・ﾟ🥢💙🖤.｡.:・ﾟ🥢💙🖤.｡.:・ﾟ🥢💙🖤.｡.:・ﾟ🥢💙🖤.｡.:・ﾟ🥢💙🖤.｡.:・ﾟ🥢"
            .chars()
            .collect();
    s.sort();
    let filter = s
        .iter()
        .filter(|&s| *s != '\u{ff9f}')
        .map(|c| c.to_string())
        .collect::<String>();
    //    let retain = s.retain(|&c| c != '\u{ff9f}');
    println!("{}", filter);
}

#[test]
fn str_sort_test() {
    str_sort();
}

#[test]
fn test_not_infinite_loop() {
    //for i in 2 as i128.. { // #[allow(clippy::unnecessary_cast)]
    for i in 0_u128.. {
        println!("{i}");
        if i >= u64::MAX.into() {
            break;
        }
    }
}

#[test]
fn test_tuple() {
    #[derive(Debug)]
    struct Tuple3<T> {
        _a: T,
        _b: T,
        _c: T,
    }
    #[derive(Debug)]
    struct Tuple3Ver2<T>(T, T, T);

    impl<T> From<(T, T, T)> for Tuple3<T> {
        fn from(value: (T, T, T)) -> Self {
            Self {
                _a: value.0,
                _b: value.1,
                _c: value.2,
            }
        }
    }

    let _tuple_mix_type = ("hello", 5, 'c');
    let tuple_one_type = ("hello", "hello", "c");

    // https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy
    // let three_tuple = tuple_one_type.clone();
    let three_tuple = tuple_one_type;

    let instance = Tuple3::<&str>::from(three_tuple);
    println!("{:?}", instance)
}

#[test]
fn test_trait_and_struct_01() {
    trait MyTrait<AT, T> {
        fn hello_world(_: T) -> AT;
    }
    struct MyStruct(String, String);

    impl MyTrait<String, MyStruct> for MyStruct {
        fn hello_world(v: MyStruct) -> String {
            format!("My name is {} {}", v.0, v.1)
        }
    }

    let my_instance = MyStruct("T".to_owned(), "Kumagai".to_owned());
    let ans = <MyStruct as MyTrait<String, MyStruct>>::hello_world(my_instance);
    println!("{ans}");
    assert_eq!(ans, "My name is T Kumagai".to_string());
}


fn example<'a>(x: &'a str) -> &'a str {
    x
}
/// 匿名ライフタイム
fn example_anonymous(x: &'_ str) -> &'_ str {
    x
}

struct Container<'a> {
    reference: &'a str,
}

impl<'a> Container<'a> {
    fn new(reference: &'a str) -> Self {
        Container { reference }
    }

    fn get_reference(&self) -> &'a str {
        self.reference
    }
}

// 匿名ライフタイムを使用する場合
impl Container<'_> {
    fn get_reference_anonymous(&self) -> &str {
        self.reference
    }
}

#[test]
fn test_reference_anoymous() {
    let text = "Hello, Rust!";
    let container = Container::new(text);
    println!("{}", container.get_reference());
    println!("{}", container.get_reference_anonymous());
}
