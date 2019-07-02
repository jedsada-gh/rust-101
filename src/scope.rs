#![allow(overflowing_literals)]
fn main() {
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // This binding *shadows* the outer one
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }

    println!("1000 as a u8 is : {}", 1000 as i32);
    println!("Hello");

}