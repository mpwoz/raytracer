use std::num::ParseFloatError;
use std::str::FromStr;
use std::string::ParseError;

use crate::matrix::Matrix;

/// Parsing matrix from string representation

impl FromStr for Matrix {
    type Err = ParseFloatError;

    /// Given a string representation create a Matrix object initialized with all the values.
    ///
    /// Example input:
    ///     | -2 | -8 |  3 |  5 |
    ///     | -3 |  1 |  7 |  3 |
    ///     |  1 |  2 | -9 |  6 |
    ///     | -6 |  7 |  7 | -9 |
    /// The above would return a 4x4 matrix struct
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Get '|' delimited strings representing rows
        let whitespace_removed: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        let rows = whitespace_removed
            .split("||")
            .map(|row| row.trim_matches('|'));

        // parse each individual row string into a vector of floats
        let mut elements = Vec::new();
        for row in rows {
            let parsed: Vec<f64> = parse_vector(row)?;
            elements.push(parsed);
        }

        Ok(Matrix::from(elements))
    }
}

/// Given a '|'-delimited list of numbers, return a vector of parsed floats.
///
/// Example input: | 1| 2|-9| 6|
/// Example output: Vec<f64> containing [1.0, 2.0, -9.0, 6.0]
fn parse_vector(rowstr: &str) -> Result<Vec<f64>, ParseFloatError> {
    let mut v = Vec::new();
    for num in rowstr.split('|') {
        let f = f64::from_str(num)?;
        v.push(f)
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_from_string() {
        let str = "
             |  6 |  4 |  4 |  4 |
             |  5 |  5 |  7 |  6 |
             |  4 | -9 |  3 | -7 |
             |  9 |  1 |  7 | -6 |";

        let m = Matrix::from_str(str).unwrap();

        let exp = Matrix::from(vec![
            vec![6., 4., 4., 4.],
            vec![5., 5., 7., 6.],
            vec![4., -9., 3., -7.],
            vec![9., 1., 7., -6.],
        ]);

        assert_eq!(m, exp);
    }

    #[test]
    #[should_panic]
    fn invalid_matrix() {
        let str = "
             |  6 |  4 |
             |  5 |  5 |  7 |  6 |
             |  9 |  1 |  7 | -6 |";
        Matrix::from_str(str).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_values() {
        let str = "
             |  6 |abc | 0  |  0 |
             |  5 |  5 |  7 |  6 |
             |  9 |  1 |  7 | -6 |";
        Matrix::from_str(str).unwrap();
    }
}
