use ordered_float::Pow;

#[derive(Debug)]
pub struct Polynomal {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Polynomal {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self { a, b, c, d }
    }

    pub fn eval(&self, s: f64) -> f64 {
        self.a + self.b * s + self.c * s.pow(2) + self.d * s.pow(3)
    }
}
