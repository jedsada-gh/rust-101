
use std::convert::From;

#[derive(Debug)]
struct User {
    first_name: String,
}

impl From<String> for User {
    fn from(user: User) -> Self {
        User { first_name: name }
    }
}

fn main() {
    let message = String::from("20scoops CNX");
    let num = User::set_name(message);
    println!("My number is {:?}", num);
}