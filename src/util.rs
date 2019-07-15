// Copyright 2016-2018 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(feature = "enable_serde")]
extern crate serde;
#[cfg(feature = "enable_serde")]
use self::serde::*;

pub use self::one_or_two::OneOrTwo;
use fnv::FnvHashSet;
use num::{BigInt, BigRational, One, Signed, Zero};
use std::borrow::Borrow;
use std::collections::{hash_set, HashSet};
use std::f64;
use std::hash::Hash;

/// # Calculates the approximate square root of the value
///
/// Calculates the approximate square root of `value`.  If the returned value is
/// `Ok(_)`, then it is guaranteed to be within `epsilon` of the actual
/// answer.  If `epsilon <= 0.0`, then `Err` is returned (the reason for the
/// bound of `0.0` is because the approximation algorithm is unable to return an
/// exact answer).  If `value < 0.0`, then `Err` is returned (`BigRational` is
/// a real valued object; it cannot represent complex values).  In both `Err`
/// cases, the value will be a `String` explaining what the error actually is.
///
/// # Parameters
///
/// - `value` - The value whose approximate square root you wish to obtain.  If
///     this is less than `0.0`, then `Err` will be returned.
/// - `epsilon` - The maximum acceptable difference between the returned value
///     and the actual value.  The returned value is in the range
///     `[actual - epsilon, actual + epsilon]`.
///
/// # Returns
///
/// If everything went as expected, then `Ok(_)` will be returned, containing
/// a value that is within `± epsilon` of the actual value.  If anything went
/// wrong, then `Err(_)` will be returned, containing a `String` outlining what
/// the problem was.
pub fn approx_square_root(value: BigRational, epsilon: BigRational) -> Result<BigRational, String> {
    if value < BigRational::zero() {
        return Err(format!(
            "approx_square_root() cannot calculate the square \
             root of negative values.  value = {}",
            value
        )
        .to_owned());
    } else if epsilon <= BigRational::zero() {
        return Err(format!(
            "approx_square_root() cannot calculate the square \
             root with a non-positive epsilon.  \
             epsilon = {}",
            epsilon
        )
        .to_owned());
    }

    // I'm going to use the Babylonian method to find the square root.  This is
    // described at
    // https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method
    // To do so, I need to have an initial seed value that is the approximate
    // square root.  This will estimate will be refined until it is within
    // epsilon of the real value.

    // Calculates seed values for all values >= 1.0.  This is used below when
    // calculating the seed value.
    #[inline]
    fn calc_seed(value: &BigRational) -> BigRational {
        let bits = value.ceil().to_integer().bits();
        let half_bits = bits / 2;
        let approximate = BigInt::one() << half_bits;
        BigRational::from_integer(approximate)
    };

    let mut x = if value >= BigRational::one() {
        calc_seed(&value)
    } else {
        // Because the value is less than one, I can't use the trick above
        // directly.  Instead, I'm going to find the reciprocal, and then do the
        // trick above, and then use the reciprocal of that as the seed.
        calc_seed(&(value.recip())).recip()
    };

    // We now have an initial seed.  Time to refine it until it is within
    // epsilon of the real value.  I'm creating two different inlined functions
    // to make it easier to do the calculations.

    #[inline]
    fn calc_next_x(value: BigRational, x: BigRational) -> BigRational {
        let two = BigRational::one() + BigRational::one();
        (x + (value / x)) / two
    };

    #[inline]
    fn calc_approx_error(value: BigRational, x: BigRational) -> BigRational {
        let two = BigRational::one() + BigRational::one();
        ((value - (x * x)) / (x * two)).abs()
    }

    while calc_approx_error(value, x) > epsilon {
        x = calc_next_x(value, x);
    }

    Ok(x)
}

/// # Calculates an approximation to the sine function
///
/// This function calculations an approximation to the sine function.  The angle
/// must be in radians.  The returned result will be within the range
/// `[actual - epsilon, actual + epsilon]`, where `actual` is the actual sine
/// of the angle.  `epsilon` must be a positive value; other values lead to
/// errors.
///
/// # Parameters
///
/// - `angle` - The angle for which you want the sine value.  This is treated as
///     being in radians.
/// - `epsilon` - The maximum acceptable difference between the returned value
///     and the actual value.  The returned value is in the range
///     `[actual - epsilon, actual + epsilon]`.
///
/// # Returns
///
/// If `epsilon > 0.0`, then the sine of `angle` is returned within an `Ok(_)`
/// variant.  Otherwise an error string is returned.
pub fn approx_sine(angle: BigRational, epsilon: BigRational) -> Result<BigRational, String> {
    // FIXME: I know that I should use the CORDIC algorithm to calculate this
    // correctly, but I don't have time to do that right now.  So, references,
    // followed by a hack
    //
    // https://pdfs.semanticscholar.org/f2a6/eef864d928b462ca2d9f7db19b4078584bf4.pdf
    // https://people.clas.ufl.edu/bruceedwards/files/paper.pdf
    // https://en.wikipedia.org/wiki/Trigonometric_functions#Basic_identities
    // https://en.wikipedia.org/wiki/CORDIC

    unimplemented!("Cem, you forgot to finish this!");
}

/// # Calculates an approximation to the cosine function
///
/// This function calculations an approximation to the cosine function.  The
/// angle must be in radians.  The returned result will be within the range
/// `[actual - epsilon, actual + epsilon]`, where `actual` is the actual cosine
/// of the angle.  `epsilon` must be a positive value; other values lead to
/// errors.
///
/// # Parameters
///
/// - `angle` - The angle for which you want the cosine value.  This is treated
///     as being in radians.
/// - `epsilon` - The maximum acceptable difference between the returned value
///     and the actual value.  The returned value is in the range
///     `[actual - epsilon, actual + epsilon]`.
///
/// # Returns
///
/// If `epsilon > 0.0`, then the cosine of `angle` is returned within an `Ok(_)`
/// variant.  Otherwise an error string is returned.
pub fn approx_cosine(angle: BigRational, epsilon: BigRational) -> Result<BigRational, String> {
    // References for the algorithm I use.
    //
    // https://pdfs.semanticscholar.org/f2a6/eef864d928b462ca2d9f7db19b4078584bf4.pdf
    // https://people.clas.ufl.edu/bruceedwards/files/paper.pdf
    // https://en.wikipedia.org/wiki/Trigonometric_functions#Basic_identities
    // https://en.wikipedia.org/wiki/CORDIC

    // This algorithm **only** works in the range [-𝞹/2, 𝞹/2]; it returns highly
    // non-sensical values for everything else.  To protect against that, we
    // return an error if the angle outside of this range.  I also require that
    // epsilon be positive, otherwise this algorithm will never terminate.
    //
    // FIXME: I should **not** be using f64::PI here; instead, I should
    // calculate it using the
    // [Chudnovsky algorithm](https://en.wikipedia.org/wiki/Chudnovsky_algorithm)
    // so that the error bounds are controlled.  However, although I can see how
    // to implement the algorithm, I don't currently know how to calculate the
    // error bounds for it.  Thus, there isn't any point in implementing it
    // right now.

    let half_pi = BigRational::from_float(f64::PI / 2.0);
    if (angle > half_pi) || (angle < -half_pi) {
        return Err(format!(
            "approx_cosine() can only handle values in the range \
             [{}, {}], but the value {} was passed in.",
            half_pi, -half_pi, angle
        ));
    } else if epsilon <= BigRational::zero() {
        return Err(format!(
            "approx_cosine() requires a positive epsilon.  \
             epsilon was {}.",
            epsilon
        ));
    }

    // I'm implementing the algorithm from the article:
    //
    // B. Tomas Johansson (2018) "An elementary algorithm to evaluate
    // trigonometric functions to high precision", International Journal of
    // Mathematical Education in Science and Technology, 49:1, 131-137,
    // DOI: 10.1080/0020739X.2017.1349943
    //
    // The preprint for this article is at
    // https://pdfs.semanticscholar.org/f2a6/eef864d928b462ca2d9f7db19b4078584bf4.pdf

    // The algorithm iteratively refines the current estimate for the cosine
    // until it is less the epsilon that is passed in.  Since the formula for
    // the error is known, we can calculate the number of iterations required
    // apriori, and then use that to iterate over the algorithm proper.  Since
    // the error is O(angle^4 / 2^(2 * k)), where k is the number of iterations,
    // I'm going to overestimate the total error, by assuming the angle is 𝞹,
    // and then solve for a k that makes the total value < epsilon.
    unimplemented!("Cem, you forgot to finish this!");
}

// returns the ascending root of a quadratic polynomial ax^2 + bx + c
pub fn quad_root_ascending(a: BigRational, b: BigRational, c: BigRational) -> Option<BigRational> {
    let determinant = b * b - a * c * BigRational::from_float(4.0).unwrap();
    let epsilon = determinant / BigRational::from_float(1000000.0).unwrap();
    if determinant <= BigRational::from_float(0.0).unwrap() {
        None
    } else if b >= BigRational::from_float(0.0).unwrap() {
        Some(
            (c * BigRational::from_float(2.0).unwrap())
                / (-b - approx_square_root(determinant, epsilon).unwrap()),
        )
    } else {
        Some(
            (-b + approx_square_root(determinant, epsilon).unwrap())
                / (a * BigRational::from_float(2.0).unwrap()),
        )
    }
}

const MIN_TIGHT_SET_CAPACITY: usize = 4;

// a HashSet that will automatically shrink down in capacity to save space
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
pub struct TightSet<T: Hash + Eq> {
    set: FnvHashSet<T>,
}

impl<T: Hash + Eq> TightSet<T> {
    pub fn new() -> TightSet<T> {
        TightSet {
            set: HashSet::with_capacity_and_hasher(MIN_TIGHT_SET_CAPACITY, Default::default()),
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.set.insert(value)
    }

    pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.set.contains(value)
    }

    pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        let success = self.set.remove(value);
        if success
            && self.set.capacity() > MIN_TIGHT_SET_CAPACITY
            && self.set.capacity() >= self.set.len() * 4
        {
            self.set.shrink_to_fit();
        }
        success
    }

    pub fn iter(&self) -> hash_set::Iter<T> {
        self.set.iter()
    }

    pub fn drain(&mut self) -> hash_set::Drain<T> {
        self.set.drain()
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn clear(&mut self) {
        if self.set.capacity() <= MIN_TIGHT_SET_CAPACITY {
            self.set.clear();
        } else {
            self.set =
                FnvHashSet::with_capacity_and_hasher(MIN_TIGHT_SET_CAPACITY, Default::default());
        }
    }
}

// a sequence of size 1 or 2 that may be iterated over and is not heap-allocated
mod one_or_two {
    #[cfg(feature = "enable_serde")]
    use super::*;

    #[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
    #[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
    pub enum OneOrTwo<T: Copy + Eq> {
        One(T),
        Two(T, T),
    }

    impl<T: Copy + Eq> OneOrTwo<T> {
        pub fn other_id(self, id: T) -> Option<T> {
            match self {
                OneOrTwo::One(id_1) if id_1 == id => None,
                OneOrTwo::Two(id_1, id_2) | OneOrTwo::Two(id_2, id_1) if id_1 == id => Some(id_2),
                _ => panic!(),
            }
        }

        pub fn iter(self) -> Iter<T> {
            Iter {
                one_or_two: self,
                index: 0,
            }
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
    #[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
    pub struct Iter<T: Copy + Eq> {
        one_or_two: OneOrTwo<T>,
        index: u8,
    }

    impl<T: Copy + Eq> Iterator for Iter<T> {
        type Item = T;
        fn next(&mut self) -> Option<T> {
            let result = match (&self.one_or_two, self.index) {
                (&OneOrTwo::One(val), 0)
                | (&OneOrTwo::Two(val, _), 0)
                | (&OneOrTwo::Two(_, val), 1) => Some(val),
                _ => None,
            };
            if result.is_some() {
                self.index += 1
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quad_root_ascending() {
        assert!(
            (quad_root_ascending(
                BigRational::from_float(1e-14).unwrap(),
                BigRational::from_float(2.0).unwrap(),
                BigRational::from_float(-1.0).unwrap()
            )
            .unwrap()
                - BigRational::from_float(0.5).unwrap())
            .abs()
                < BigRational::from_float(1e-7)
        );
        assert!(
            (quad_root_ascending(
                BigRational::from_float(0.0).unwrap(),
                BigRational::from_float(2.0).unwrap(),
                BigRational::from_float(-1.0).unwrap()
            )
            .unwrap()
                - BigRational::from_float(0.5).unwrap())
            .abs()
                < BigRational::from_float(1e-7)
        );
        assert!(
            (quad_root_ascending(
                BigRational::from_float(100.0).unwrap(),
                BigRational::from_float(-1.0).unwrap(),
                BigRational::from_float(-1e-16).unwrap()
            )
            .unwrap()
                - BigRational::from_float(0.01).unwrap())
            .abs()
                < BigRational::from_float(1e-7)
        );
        assert!(quad_root_ascending(
            BigRational::from_float(0.0).unwrap(),
            BigRational::from_float(-2.0).unwrap(),
            BigRational::from_float(1.0).unwrap()
        )
        .unwrap()
        .is_infinite());
        assert!(quad_root_ascending(
            BigRational::from_float(-3.0).unwrap(),
            BigRational::from_float(0.0).unwrap(),
            BigRational::from_float(-1.0).unwrap()
        )
        .is_none());
        assert!(quad_root_ascending(
            BigRational::from_float(1.0).unwrap(),
            BigRational::from_float(1.0).unwrap(),
            BigRational::from_float(1.0).unwrap()
        )
        .is_none());
    }
}
