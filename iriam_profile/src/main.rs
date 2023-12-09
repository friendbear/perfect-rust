
type S = LiveStreamer;
#[derive(Debug, Default, Clone)]
struct LiveStreamer {
    name: Option<String>,
    mark: Option<String>,
    x_name: Option<String>,
}
impl LiveStreamer {
    fn _printer(&self) {
        println!(
            "{}{}{} Liked♡",
            self.name.clone().unwrap_or_default(),
            self.mark.clone().unwrap_or_default(),
            self.x_name.clone().unwrap_or_default()
        )
    }
}
struct Builder(LiveStreamer);
fn main() {
    let streamer = vec![
        Builder::new().with_name("eL(神様)").build(),
        Builder::new()
            .with_name("はしちゃん")
            .with_mark("🥢💙🖤")
            .build(),
        Builder::new().with_mark("☁️🎀").build(),
        Builder::new().with_mark("📘📗🌼").build(),
        Builder::new().with_mark("🐈‍⬛💜.*･").build(),
    ];
    let printer = |s: &S| {
        println!(
            "{} Liked♡",
            s.name.clone().unwrap_or_default()
                + s.mark.clone().unwrap_or_default().as_str()
                + s.x_name.clone().unwrap_or_default().as_str()
        )
    };
    streamer.iter().for_each(|s| printer(s));
    streamer.into_iter().for_each(|s| printer(&s));
}
impl Builder {
    fn new() -> Self {
        Builder(LiveStreamer::default())
    }
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
fn test_loop() {
    for i in 2 as i128.. {
        println!("{i}");
        if i >= i16::MAX.into() {
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
        _c: T
    }
    #[derive(Debug)]
    struct Tuple3Ver2<T>(T,T,T);

    impl<T> From<(T, T, T)> for Tuple3<T> {
        fn from(value: (T, T, T)) -> Self {
            Self {
                _a: value.0,
                _b: value.1,
                _c: value.2
            }
        }
    }


    let _tuple_mix_type = ("hello", 5, 'c');
    let tuple_one_type = ("hello", "hello", "c");

    let three_tuple = tuple_one_type.clone();

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