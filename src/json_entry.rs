use serde::{Deserialize, Serialize};

use crate::bmi::Bmi;
#[derive(Serialize, Deserialize)]
pub struct Bmi_Entry {
    bmi: Bmi,
    time: u64
}

impl Bmi_Entry {
    pub fn new(bmi: Bmi, time: u64) -> Bmi_Entry {
        Bmi_Entry{ bmi, time}
    }
}