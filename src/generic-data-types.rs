#[derive(Debug)]
struct Point<T> {
    number_1: T,
    number_2: T
}

impl<T> Point<T> {
    fn get_number_1(&self) -> &T {
        &self.number_1
    }
}

#[derive(Debug)]
struct Scoops<I, S> {
    full_name: S,
    age: I
}

impl<I, S> Scoops<I, S> {
    fn get_full_name(&self) -> &S {
        &self.full_name
    }

    fn get_age(&self) -> &I {
        &self.age
    }
}

fn main() {
    let integer = Point{ number_1: 4, number_2: 5 };
    let scoops = Scoops{ full_name: "20scoops CNX", age: 3};
    println!("scoops: {:?}", scoops);
    println!("fulle name: {:?}\tage: {:?}", scoops.get_full_name(), scoops.get_age());
    println!("integers: {:?}", integer.get_number_1());
}