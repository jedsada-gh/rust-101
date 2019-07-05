fn main() {
    let point = 76;

    if point >= 80 {
        println!("A")
    } else if point >= 75 {
        println!("B+")
    } else if point >= 70 {
        println!("B")
    } else if point >= 65 {
        println!("C+")
    } else if point >= 60 {
        println!("C")
    } else if point >= 50 {
        println!("D")
    } else {
        println!("F")
    }

    let index = 1;
    let result = if index % 2 == 0 {
        "even"
    } else {
        "odd"
    };

    println!("result is {}", result);

    let some_value = 5;
    match some_value {
        0 => println!("value is zero"),
        1 => println!("value is one"),
        5 => {
            println!("value is five")
        },
        _ => (),
    }
}