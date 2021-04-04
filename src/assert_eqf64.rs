/// Float equality assertion macro, simplified (for learning) version of https://github.com/ashleygwilliams/assert_approx_eq/blob/master/src/lib.rs
#[macro_export]
macro_rules! assert_eqf64 {
        ($a: expr, $b: expr) => {{
            let (a, b) = (&$a, &$b); // references just for easy typing below (no $)

            let delta = (a - b).abs();
            let eps = f64::EPSILON; // if we used our own epsilon we could make this work for f32 too

            assert!(
                delta < eps,
                "Equality check failed, {} != {}, difference was {} which is larger than tolerance of {}",
                a, b, delta, eps
            )
        }};
    }

#[cfg(test)]
mod tests {
    #[test]
    fn test_equal() {
        assert_eqf64!(2_f64, 2_f64);
    }

    #[test]
    #[should_panic]
    fn test_not_equal() {
        let a = 1_f64;
        let b = 3_f64;
        assert_eqf64!(a, b);
    }
}
