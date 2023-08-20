trait Streamer {
    fn talk(&self);
    fn play(&self);
}

trait Artist {
    fn draw(&self);
}

// marker trait.
trait Creator: Streamer + Artist {}

struct Person;

impl Streamer for Person {
    fn talk(&self) {
        println!("Person is talking.");
    }
    fn play(&self) {
        println!("Person is playing music.");
    }
}

impl Artist for Person {
    fn draw(&self) {
        println!("Person is drawing.");
    }
}

#[test]
fn sub_trait() {
    let person = Person {};
    person.draw();
    person.play();
    person.talk();
}
