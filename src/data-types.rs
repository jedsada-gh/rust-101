fn main() {

    // let && const
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    let f: bool = false;
    let company = "20scoops";

    println!("{}", company);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", tup.0);
    println!("The value of y is: {}", y);

    let a = [1, 2, 3, 4, 5];
    println!("Arrays a is: {:?}", a);
    println!("Value index 0 in a is: {}", a[0]);

    let dup = [3; 5];
    println!("Arrays a is: {:?}", dup);
}