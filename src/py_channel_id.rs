#[cfg(test)]
use proptest_derive::Arbitrary;

use pyo3::prelude::*;
use pyo3::CompareOp;
use pyo3::PyObjectProtocol;

use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

/// # The identifier for a channel
///
/// Channel identifiers are simply thin wrappers around `u128` instances.  All
/// values are accepted and valid.
///
///
/// # Elements
///
/// * value - Just a u128.
#[pyclass]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PyChannelIdentifier {
    value: u128,
}

impl fmt::Display for PyChannelIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PyChannelIdentifier{{value: {}}}", self.value)
    }
}

#[pymethods]
impl PyChannelIdentifier {
    #[new]
    pub fn __new__(obj: &PyRawObject, value: u128) -> PyResult<()> {
        obj.init(|| PyChannelIdentifier::new(value))
    }

    #[getter]
    fn get_id(&self) -> PyResult<u128> {
        Ok(self.value)
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for PyChannelIdentifier {
    fn __str__(&'p self) -> PyResult<String> {
        Ok(format!("{}", self))
    }

    fn __repr__(&'p self) -> PyResult<String> {
        Ok(format!("{}", self))
    }

    fn __hash__(&'p self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&'p self, other: &'p PyChannelIdentifier, op: CompareOp) -> PyResult<bool> {
        let result = self.cmp(other);

        match op {
            CompareOp::Lt => Ok(result == Ordering::Less),
            CompareOp::Le => Ok((result == Ordering::Less) || (result == Ordering::Equal)),
            CompareOp::Eq => Ok(result == Ordering::Equal),
            CompareOp::Ne => Ok(result != Ordering::Equal),
            CompareOp::Gt => Ok(result == Ordering::Greater),
            CompareOp::Ge => Ok((result == Ordering::Greater) || (result == Ordering::Equal)),
        }
    }
}

impl PyChannelIdentifier {
    fn new(value: u128) -> PyChannelIdentifier {
        PyChannelIdentifier { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::hash::Hasher;
    use std::u128;

    proptest! {
        // Just a crash test.  If this fails, then there are far more serious
        // bugs that need to be fixed.
        #[test]
        fn test_new(argument: u128) {
            // This should never crash, regardless of the incoming value.
            PyChannelIdentifier::new(argument);
        }

        #[test]
        fn test_copy_clone(argument: u128) {
            let a = PyChannelIdentifier::new(argument);
            let b = a.clone();
            let a_ref = &a as *const PyChannelIdentifier;
            let b_ref = &b as *const PyChannelIdentifier;

            prop_assert_eq!(a, b);
            prop_assert_ne!(a_ref, b_ref);
        }

        // Yet another crash test.  Not particularly worried about it though.
        #[test]
        fn test_display_debug(argument: u128) {
            let a = PyChannelIdentifier::new(argument);

            println!("{} = {:?}", a, a);
        }

        #[test]
        fn test_comparisons(a: u128, b: u128) {
            let first = PyChannelIdentifier::new(a);
            let second = PyChannelIdentifier::new(b);

            prop_assert_eq!(a < b, first < second);
            prop_assert_eq!(a <= b, first <= second);
            prop_assert_eq!(a != b, first != second);
            prop_assert_eq!(a == b, first == second);
            prop_assert_eq!(a >= b, first >= second);
            prop_assert_eq!(a > b, first > second);
        }

        #[test]
        fn test_hash(a: u128, b: u128) {
            let first = PyChannelIdentifier::new(a);
            let mut first_hasher = DefaultHasher::new();
            first.hash(&mut first_hasher);
            let fh = first_hasher.finish();

            let second = PyChannelIdentifier::new(b);
            let mut second_hasher = DefaultHasher::new();
            second.hash(&mut second_hasher);
            let sh = second_hasher.finish();

            let third = PyChannelIdentifier::new(a);
            let mut third_hasher = DefaultHasher::new();
            third.hash(&mut third_hasher);
            let th = third_hasher.finish();

            if a == b {
                prop_assert_eq!(fh, sh);
                prop_assert_eq!(fh, th);
                prop_assert_eq!(sh, th);
            } else {
                prop_assert_ne!(fh, sh);
                prop_assert_eq!(fh, th);
                prop_assert_ne!(sh, th);
            }
        }
    }
}
