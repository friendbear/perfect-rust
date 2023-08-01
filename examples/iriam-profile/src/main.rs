use std::fmt::{Display, Error, Formatter};
type S = LiveStreamer;
#[derive(Debug, Default)]
struct LiveStreamer {
    name: Option<String>,
    mark: Option<String>,
    x_name: Option<String>,
}
fn main() {
    let print = |s: S| {
        println!(
            "{} Liked♡",
            s.name.unwrap_or_default()
                + &s.mark.unwrap_or_default()
                + &s.x_name.unwrap_or_default()
        )
    };
    let mut st = vec![
        S::new(Some("eL(神様)".to_owned()), None, None),
        S::new(None, Some("🥢💙🖤".to_owned()), None),
        S::new(None, Some("☁️🎀".to_owned()), None),
        S::new(None, Some("📘📗🌼".to_owned()), None),
        S::new(None, Some("🐈‍⬛💜.*･".to_owned()), None),
    ];
    st.into_iter().for_each(|s| print(s));
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
