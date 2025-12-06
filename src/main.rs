mod math;

fn main() {
    // clippy should be allowed

    // clippie should not be allowed

    println!("Hello, world!");

    let (x, y) = (2, 3);

    println!("{x} + {y} = {}", math::add(x, y));
}
