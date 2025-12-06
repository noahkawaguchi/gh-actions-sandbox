mod math;

fn main() {
    println!("Hello, world!");

    // This is just a comment so that there will be a change

    let (x, y) = (2, 3);

    println!("{x} + {y} = {}", math::add(x, y));
}
