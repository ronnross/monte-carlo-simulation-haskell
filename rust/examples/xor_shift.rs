extern crate rand;
extern crate monte_carlo;

use monte_carlo::pi;
use rand::{Rng, SeedableRng};

fn main() {
    let limit = std::env::args()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let mut rng = get_rng();

    println!("{}", pi(&mut rng, limit));
}

fn get_rng() -> rand::XorShiftRng {
    let mut default_rng = rand::thread_rng();

    let buf = [default_rng.next_u32(),
               default_rng.next_u32(),
               default_rng.next_u32(),
               default_rng.next_u32()];

    rand::XorShiftRng::from_seed(buf)
}
