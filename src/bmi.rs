use serde::{Deserialize, Serialize};

use crate::height::Height;
use crate::weight::Weight;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bmi {
    value: f64,
    height: Height,
    weight: Weight,
}

impl Bmi {
    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn new(bmi: f64, height: Height, weight: Weight) -> Bmi {
        Bmi {
            value: bmi,
            height,
            weight,
        }
    }

    pub fn height(&self) -> Height {
        self.height
    }

    pub fn weight(&self) -> Weight {
        self.weight
    }
}
