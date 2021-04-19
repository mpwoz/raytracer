use crate::assert_eqf64;
use crate::object::Object;

pub struct Intersection<'a, T>
    where
        T: Object,
{
    pub t: f64,
    pub object: &'a T,
}

impl<T> Intersection<'_, T>
    where
        T: Object,
{
    pub fn new(t: f64, object: &T) -> Intersection<T> {
        Intersection { t, object }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    use crate::sphere::Sphere;

    use super::*;

    #[test]
    fn test() {
        let sphere = Sphere::new();
        let i = Intersection::new(1.0, &sphere);

        assert_eqf64!(i.t, 1_f64);
        assert_eq!(i.object.transform, Matrix::transformation());
    }
}
