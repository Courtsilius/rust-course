#[derive(Debug)]
pub struct Bmi {
    value: f64,
}

impl Bmi {
    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn new(bmi: f64) -> Bmi {
        Bmi { value: bmi }
    }
}
