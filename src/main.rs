mod math;

fn main() {
    println!("Hello, world!");

    // This is just a comment

    let (x, y) = (2, 3);

    println!("{x} + {y} = {}", math::add(x, y));
}
