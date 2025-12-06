mod math;

fn main() {
    println!("Hello, world!");

    let (x, y) = (2, 3);

    println!("{x} + {y} = {}", math::add(x, y));
}
