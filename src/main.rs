mod math;

fn main() {
    println!("Hello, world!");

    let (x, y) = (2, 3);

    println!("{x} + {y} = {}", math::add(x, y));

    let num = math::random_number_le(90);

    println!("{num} <= 90");
}
