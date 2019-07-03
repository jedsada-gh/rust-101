#[derive(Debug)]
struct User<'s> {
    first_name: &'s str,
    last_name: &'s str,
    age: u8
}

#[derive(Debug)]
struct Pair<'s>(&'s str, &'s str);

fn main() {
    let user = User {
        first_name: "20scoops",
        last_name: "CNX",
        age: 3
    };
    println!("{:?}", user);

    // destructure_structures
    let User {first_name, last_name, age } = user;
    println!("fist name: {}, last name: {}, age: {:?}", first_name, last_name, age);

    // Pair
    let pair = Pair("20scoops", "CNX");
    println!("values: {:?}", pair);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
}