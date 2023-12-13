/**
* My Rust version hello world.
* see: <https://users.rust-lang.org/t/rust-lang-version-hello-world-starting-here>
*/
fn main() {
    trait MyTrait<AT, T> {
        fn hello_world(_: T) -> AT;
    }
    struct MyStruct(String, String);

    impl MyTrait<String, MyStruct> for MyStruct {
        fn hello_world(v: MyStruct) -> String {
            format!("Hello world, My name is {} {}", v.0, v.1)
        }
    }

    let my_instance = MyStruct("T".to_owned(), "Kumagai".to_owned());
    let hello_world_string = <MyStruct as MyTrait<String, MyStruct>>::hello_world(my_instance);
    
    println!("{hello_world_string}");
    assert_eq!(hello_world_string, "Hello world, My name is T Kumagai".to_string());
}
