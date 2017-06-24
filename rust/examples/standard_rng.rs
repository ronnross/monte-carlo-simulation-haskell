extern crate rand;
extern crate monte_carlo;

use monte_carlo::pi;

fn main() {
    let limit = std::env::args()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let mut rng = rand::thread_rng();

    println!("{}", pi(&mut rng, limit));
}
