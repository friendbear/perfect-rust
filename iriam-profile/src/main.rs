use std::fmt::{Display, Error, Formatter};
type S = LiveStreamer;
#[derive(Debug, Default, Clone)]
struct LiveStreamer {
    name: Option<String>,
    mark: Option<String>,
    x_name: Option<String>,
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
    let printer = |s: S| {
        println!(
            "{} Liked♡",
            s.name.unwrap_or_default()
                + &s.mark.unwrap_or_default()
                + &s.x_name.unwrap_or_default()
        )
    };
    streamer.into_iter().for_each(printer);
}
impl Builder {
    fn new() -> Self {
        Builder(LiveStreamer::default())
    }
    fn with_name(&self, name: &str) -> Self {
        Builder(LiveStreamer {
            name: Some(name.to_string()),
            ..self.0.clone()
        })
    }
    fn with_mark(&self, mark: &str) -> Self {
        Builder(LiveStreamer {
            mark: Some(mark.to_string()),
            ..self.0.clone()
        })
    }
    fn build(self) -> LiveStreamer {
        self.0
    }
}
impl LiveStreamer {
    #[allow(dead_code)]
    fn new(name: Option<String>, mark: Option<String>, x_name: Option<String>) -> Self {
        Self { name, mark, x_name }
    }
}
impl Display for LiveStreamer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "name: {}, mark: {} x: {}",
            self.name.as_ref().unwrap(),
            self.mark.as_ref().unwrap(),
            self.x_name.as_ref().unwrap()
        )?;
        Ok(())
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
