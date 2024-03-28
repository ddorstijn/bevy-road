use ordered_float::Pow;

#[derive(Debug)]
pub struct Polynomal {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl Polynomal {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self { a, b, c, d }
    }

    pub fn eval(&self, s: f32) -> f32 {
        self.a + self.b * s + self.c * s.pow(2) + self.d * s.pow(2)
    }
}
