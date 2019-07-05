use std::collections::HashMap;

fn main() {
    let mut employees = HashMap::new();
    employees.insert(String::from("Android Developer"), 3);
    employees.insert(String::from("Smart Contract Developer"), 2);
    println!("employees: {:?}", employees);
    
    let android_dev_count = employees.get("Android Developer");
    match android_dev_count {
        Some(value) => println!("android dev count: {}", value),
        None => println!("Not found android dev"),
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    for (key, value) in & scores {
        println!("{}: {}", key, value);
    }
}