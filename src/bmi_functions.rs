// calculates bmi based on height and weight
pub mod bmi_mod {
    use crate::{Bmi, BmiError, Height, Weight};
    pub fn calculate_bmi(height: Height, weight: Weight) -> Result<Bmi, BmiError> {
        if height.value() <= 0.0 {
            return Err(BmiError::HeightCannotBeZeroOrSmaller);
        }

        if weight.0 <= 0.0 {
            return Err(BmiError::WeightCannotBeZeroOrSmaller);
        }

        let bmi = weight.0 / (f64::powf(height.value(), 2.0));
        Ok(Bmi::new(bmi, height, weight))
    }
}
