mod math;

fn main() {
    let (x, y) = (2, 6);
    println!("{x} + {y} = {}", math::add(x, y));

    // Lint check: manual midpoint
    // let mid = (x + y) / 2;
    // println!("midpoint of {x} and {y} is {mid}");
    let mid = x.midpoint(y);
    println!("midpoint of {x} and {y} is {mid}");

    let num = math::random_number_le(90);
    println!("{num} <= 90");
}
