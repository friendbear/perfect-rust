use std::marker::Copy;
use std::ops::Add;
#[derive(Debug, Clone)]
struct Customer<T> {
    id: T,
    name: String,
    address: String,
    email: String,
}

impl<T> Customer<T> where T: Add + Copy {}
