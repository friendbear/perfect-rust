use std::marker::Copy;
use std::ops::Add;
#[allow(dead_code)]
struct Customer<T> {
    id: T,
    name: String,
    address: String,
    email: String,
}

impl<T> Customer<T> where T: Add + Copy {}
