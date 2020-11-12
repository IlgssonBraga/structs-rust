fn main() {
    let react1 = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area(react1));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
