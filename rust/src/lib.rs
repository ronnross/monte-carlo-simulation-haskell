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

    fn weight(&self) -> i64 {
        if (self.x * self.x + self.y * self.y).sqrt() <= 1.0 {
            1
        } else {
            0
        }
    }
}

pub fn pi<T>(gen: &mut T, limit: usize) -> f64
    where T: Rng
{
    let mut points = Vec::with_capacity(limit);

    for _ in 0..limit {
        points.push(Point::random(gen));
    }

    let total_weight = points.iter().fold(0, |x, y| x + y.weight()) as f64;

    total_weight / points.len() as f64 * 4.0
}
