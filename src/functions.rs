fn main() {
    print!("Company: {}", get_name());
    set_name("CNX");
    println!("Number is {}", number_five())
}

fn set_name(m_name : &str) {
    println!("\t{}", m_name);
}

fn get_name() -> String {
    return String::from("20scoops")
}

fn number_five() -> i32 {
    5
}