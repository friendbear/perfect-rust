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
    let st = vec![
        Builder::new().with_name("eL(神様)").build(),
        Builder::new().with_name("はしちゃん").with_mark("🥢💙🖤").build(),
        Builder::new().with_mark("☁️🎀").build(),
        Builder::new().with_mark("📘📗🌼").build(),
        Builder::new().with_mark("🐈‍⬛💜.*･").build(),
    ];
    let closure = |s: S| {
        println!(
            "{} Liked♡",
            s.name.unwrap_or_default()
                + &s.mark.unwrap_or_default()
                + &s.x_name.unwrap_or_default()
        )
    };
    st.into_iter().for_each(|s| closure(s));
}
impl Builder {
    fn new() -> Builder {
        Builder(LiveStreamer::default())
    }
    fn with_name(&self, name: &str) -> Builder {
        Builder(LiveStreamer {
            name: Some(name.to_string()),
            ..self.0.clone()
        })
    }
    fn with_mark(&self, mark: &str) -> Builder {
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
