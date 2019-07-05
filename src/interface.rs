#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    age: u32
}

impl User {
    fn get_full_name(self) -> String {
        let full_name = format!("{}\t{}", self.first_name, self.last_name);
        return full_name;
    }
}

fn main() {
    let mut vec = Vec::new();
    vec.push("20scoops");
    vec.push("CNX");
    println!("{:?}", vec);

    let mut user = User {
        first_name : vec[0].to_string(),
        last_name : vec[1].to_string(),
        age: 3
    };
    user.age = 10;
    println!("age: {:?}", user.age);
    println!("full name: {}", user.get_full_name());
}
