fn main() {

    // for loop
    for number in 1..4 {
        println!("{}!", number);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    // loop
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Counter: {}", counter);
        if counter == 9_999 {
            break
        }
    }

    // while loop
    let mut index = 0;
    while index <= 1_000 {
        println!("index: {}", index);
        index += 1;
    }
}