mod generics;
fn main() {
    let json_reader = generics::traits_impl::JsonReaderImpl::<u32> {
        phantom: std::marker::PhantomData,
    };
    println!("Hello, world!");
}
