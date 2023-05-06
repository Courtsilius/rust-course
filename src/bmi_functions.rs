// calculates bmi based on height and weight
pub mod bmi_mod {
    use crate::{Bmi, BmiError, Height, Weight};
    pub fn calculate_bmi(height: Height, weight: Weight) -> Result<Bmi, BmiError> {
        if height.0 <= 0.0 {
            return Err(BmiError::HeightCannotBeZeroOrSmaller);
        }

        let bmi = weight.0 / (f64::powf(height.0, 2.0));
        Ok(Bmi::new(bmi))
    }
}
