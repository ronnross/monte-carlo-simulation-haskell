extern crate rand;

use rand::Rng;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn random<T>(gen: &mut T) -> Self
        where T: Rng
    {
        Point {
            x: gen.gen(),
            y: gen.gen(),
        }
    }

    fn weight(&self) -> f64 {
        if (self.x * self.x + self.y * self.y).sqrt() <= 1.0 {
            1.0
        } else {
            0.0
        }
    }
}

pub fn pi<T>(gen: &mut T, limit: usize) -> f64
    where T: Rng
{
    let mut total_weight = 0.0;

    for _ in 0..limit {
        total_weight += Point::random(gen).weight();
    }

    total_weight / limit as f64 * 4.0
}
