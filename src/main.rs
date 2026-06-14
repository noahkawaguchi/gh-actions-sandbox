mod env;
mod math;

fn main() {
    let (x, y) = (2, 6);

    match math::try_add(x, y) {
        Ok(z) => println!("{x} + {y} = {z}"),
        Err(e) => println!("Oh no! {e}"),
    }

    // Lint check: manual midpoint
    // let mid = (x + y) / 2;
    // println!("midpoint of {x} and {y} is {mid}");
    let mid = x.midpoint(y);
    println!("midpoint of {x} and {y} is {mid}");

    let num = math::random_number_le(90);
    println!("{num} <= 90");
}
