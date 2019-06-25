fn main() {
    let x = 5;

    // this complie error because x not mutable
    // x = x + 2;

    // keyword mut is mutable
    let mut y = x + 10;
    y = y * 10;

    // constant variable
    const Z: u32 = 10;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of Z is: {}", Z);
}