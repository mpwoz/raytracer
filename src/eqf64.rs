/// Function to test for float equality
pub fn eq_f64(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        assert!(eq_f64(2_f64, 2_f64));
    }
}
