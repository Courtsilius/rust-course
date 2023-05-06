#[cfg(test)]
mod tests {
    use crate::{calculate_bmi, BmiError, Height, Weight};
    use float_eq::assert_float_eq;

    #[test]
    fn test_bmi() {
        let result = calculate_bmi(Height(1.69), Weight(69.0)).unwrap();
        assert_float_eq!(result.value(), 24.15, abs <= 0.01);
    }

    #[test]
    fn test_bmi_fail() {
        let res = calculate_bmi(Height(0.0), Weight(1.69));
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, BmiError::HeightCannotBeZeroOrSmaller);
    }
}
