fn main() {
    let reference = &4;

    println!("value: {:?}", reference);

    let address = format!("{:p}", reference);
    println!("{}", address);

    match reference {
        &val => println!("value: {:?}", val)
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 30;

    println!("`_not_a_reference`: {:?}", _not_a_reference);

    println!("`_is_a_reference`: {:?}", _is_a_reference);

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r)
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
    println!("mut_value: {:?}", mut_value);
}