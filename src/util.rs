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
use log;
use noisy_float::prelude::*;
use std::borrow::Borrow;
use std::collections::{hash_set, HashSet};
use std::f64;
use std::hash::Hash;

/// # Increases `value` by the least amount possible.
///
/// This function will increase `value` by the least possible amount to ensure
/// that the output is greater than thn `value`.  If `value == f64::MAX`, or if
/// `!value.is_finite()`, then `value` will be returned unchanged.
///
/// So why does this function exist at all?  Time must **always** increase;
/// setting time to the same date as the current date ensures that you will have
/// serious issues at some point during your simulation.  This function 'solves'
/// the problem by returning the least representable value that is greater than
/// the input.  This is probably a bad idea, but I'm out of good ideas, so I'm
/// doing this.
///
/// ```text
///            ▄▄▄▄▄        ▄▄     ▄▄▄   ▄▄     ▄▄▄▄   ▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄
///            ██▀▀▀██     ████    ███   ██   ██▀▀▀▀█  ██▀▀▀▀▀▀  ██▀▀▀▀██
///            ██    ██    ████    ██▀█  ██  ██        ██        ██    ██
///            ██    ██   ██  ██   ██ ██ ██  ██  ▄▄▄▄  ███████   ███████
///            ██    ██   ██████   ██  █▄██  ██  ▀▀██  ██        ██  ▀██▄
///            ██▄▄▄██   ▄██  ██▄  ██   ███   ██▄▄▄██  ██▄▄▄▄▄▄  ██    ██
///            ▀▀▀▀▀     ▀▀    ▀▀  ▀▀   ▀▀▀     ▀▀▀▀   ▀▀▀▀▀▀▀▀  ▀▀    ▀▀▀
///
///
///
///                     ▄▄      ▄▄  ▄▄▄▄▄▄   ▄▄        ▄▄
///                     ██      ██  ▀▀██▀▀   ██        ██
///                     ▀█▄ ██ ▄█▀    ██     ██        ██
///                      ██ ██ ██     ██     ██        ██
///                      ███▀▀███     ██     ██        ██
///                      ███  ███   ▄▄██▄▄   ██▄▄▄▄▄▄  ██▄▄▄▄▄▄
///                      ▀▀▀  ▀▀▀   ▀▀▀▀▀▀   ▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀
///
///
///
///  ▄▄▄▄▄▄      ▄▄▄▄    ▄▄▄▄▄▄     ▄▄▄▄▄▄   ▄▄▄   ▄▄    ▄▄▄▄      ▄▄▄▄    ▄▄▄   ▄▄
///  ██▀▀▀▀██   ██▀▀██   ██▀▀▀▀██   ▀▀██▀▀   ███   ██  ▄█▀▀▀▀█    ██▀▀██   ███   ██
///  ██    ██  ██    ██  ██    ██     ██     ██▀█  ██  ██▄       ██    ██  ██▀█  ██
///  ███████   ██    ██  ███████      ██     ██ ██ ██   ▀████▄   ██    ██  ██ ██ ██
///  ██  ▀██▄  ██    ██  ██    ██     ██     ██  █▄██       ▀██  ██    ██  ██  █▄██
///  ██    ██   ██▄▄██   ██▄▄▄▄██   ▄▄██▄▄   ██   ███  █▄▄▄▄▄█▀   ██▄▄██   ██   ███
///  ▀▀    ▀▀▀   ▀▀▀▀    ▀▀▀▀▀▀▀    ▀▀▀▀▀▀   ▀▀   ▀▀▀   ▀▀▀▀▀      ▀▀▀▀    ▀▀   ▀▀▀
/// ```
///
/// This function assumes that the bit layout of an `f64` is as specified in
/// IEEE 754-2008 (https://en.wikipedia.org/wiki/IEEE_754-2008_revision)
/// for a `binary64`
/// (https://en.wikipedia.org/wiki/Double-precision_floating-point_format) type.
///
/// Because of this assumption, if the layout is not identical to a
/// `binary64` type, this function **will** cause serious havoc to your program,
/// probably in ways that will be hard to trace.  There are a series
/// `debug_assert()!` statements that try to limit the possible damage, but they
/// aren't guaranteed to work.  Be careful!
///
/// # Parameters
///
/// - `value` - A `f64` that you want to bump up to the next representable
///     value.  Provided this is both finite and less than `f64::MAX`, the
///     returned value will be the least representable value strictly greater
///     than the input.  If `value` is not finite, or if it is equal to
///     `f64::MAX`, then this will be returned unchanged.
///
/// # Return value
///
/// The `f64` that is the least representable value strictly greater than
/// `value` if `(value.is_finite()) && (value < f64::MAX)`, or `value` if not.
#[allow(dead_code)]
#[inline]
pub(crate) fn bump_f64(value: f64) -> f64 {
    if (value.is_finite()) && (value < f64::MAX) {
        // NOTES: I'm assuming that the layout of an f64 is now and will always
        // be exactly that specified by IEEE 754, which can be found at
        // https://en.wikipedia.org/wiki/Double-precision_floating-point_format.
        // If that changes, then this will silently break, causing all kinds of
        // havoc.  You have been warned!
        const MASK: u64 = (1 << 63) - 1;

        // The trick that I'm using here is pretty simple; the fractional bits
        // are all stored in the least significant bits of the f64, and they are
        // stored as an unsigned integer. The next 11 most significant bits hold
        // the exponent, once again as an unsigned integer.  When we add one,
        // either the new fractional bits will contain the complete change, or
        // the overflow will cause the fractional bits to be 0, and the exponent
        // to increase by one bit.  In both cases, we're OK.  The only time this
        // trick can break down is when value is >= f64::MAX... which we've
        // already proven isn't true...
        let bits = value.to_bits();
        let result = ((!MASK) & bits) | (MASK & (bits + 1));

        log::trace!("Bumped {:?} to {:?}.", value, result);

        f64::from_bits(result)
    } else {
        value
    }
}

/// # Executes and returns the bumped value of the incoming operation.
///
/// Executes `operation(a, b)`, testing the returned result for equality to
/// both `a` and `b`.  If the result is equal to either `a` or `b`, then
/// `bump_f64(result)` is returned, otherwise the result is returned unchanged.
///
/// ```text
///            ▄▄▄▄▄        ▄▄     ▄▄▄   ▄▄     ▄▄▄▄   ▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄
///            ██▀▀▀██     ████    ███   ██   ██▀▀▀▀█  ██▀▀▀▀▀▀  ██▀▀▀▀██
///            ██    ██    ████    ██▀█  ██  ██        ██        ██    ██
///            ██    ██   ██  ██   ██ ██ ██  ██  ▄▄▄▄  ███████   ███████
///            ██    ██   ██████   ██  █▄██  ██  ▀▀██  ██        ██  ▀██▄
///            ██▄▄▄██   ▄██  ██▄  ██   ███   ██▄▄▄██  ██▄▄▄▄▄▄  ██    ██
///            ▀▀▀▀▀     ▀▀    ▀▀  ▀▀   ▀▀▀     ▀▀▀▀   ▀▀▀▀▀▀▀▀  ▀▀    ▀▀▀
///
///
///
///                     ▄▄      ▄▄  ▄▄▄▄▄▄   ▄▄        ▄▄
///                     ██      ██  ▀▀██▀▀   ██        ██
///                     ▀█▄ ██ ▄█▀    ██     ██        ██
///                      ██ ██ ██     ██     ██        ██
///                      ███▀▀███     ██     ██        ██
///                      ███  ███   ▄▄██▄▄   ██▄▄▄▄▄▄  ██▄▄▄▄▄▄
///                      ▀▀▀  ▀▀▀   ▀▀▀▀▀▀   ▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀
///
///
///
///  ▄▄▄▄▄▄      ▄▄▄▄    ▄▄▄▄▄▄     ▄▄▄▄▄▄   ▄▄▄   ▄▄    ▄▄▄▄      ▄▄▄▄    ▄▄▄   ▄▄
///  ██▀▀▀▀██   ██▀▀██   ██▀▀▀▀██   ▀▀██▀▀   ███   ██  ▄█▀▀▀▀█    ██▀▀██   ███   ██
///  ██    ██  ██    ██  ██    ██     ██     ██▀█  ██  ██▄       ██    ██  ██▀█  ██
///  ███████   ██    ██  ███████      ██     ██ ██ ██   ▀████▄   ██    ██  ██ ██ ██
///  ██  ▀██▄  ██    ██  ██    ██     ██     ██  █▄██       ▀██  ██    ██  ██  █▄██
///  ██    ██   ██▄▄██   ██▄▄▄▄██   ▄▄██▄▄   ██   ███  █▄▄▄▄▄█▀   ██▄▄██   ██   ███
///  ▀▀    ▀▀▀   ▀▀▀▀    ▀▀▀▀▀▀▀    ▀▀▀▀▀▀   ▀▀   ▀▀▀   ▀▀▀▀▀      ▀▀▀▀    ▀▀   ▀▀▀
/// ```
///
/// Because this function tests for equality, it won't produce the results you
/// expect when doing subtraction or other negating operations.  **Only** use
/// this on operations where you expect the result to be greater than either of
/// the operands!
///
/// # Parameters
///
/// - `operation` - A binary operation that will be executed on `a` and `b`.
///     `a` will always be the first operand, and `b` will always be the second,
///     so if the order matters, you can count on this order.
/// - `a` - The first operad to `operation`.  May be any representable value for
///     a `f64`; in particular `NaN` and infinity are handled correctly.
/// - `b` - The second operad to `operation`.  May be any representable value
///     for a `f64`; in particular `NaN` and infinity are handled correctly.
///
/// # Return value
///
/// If the result of `operation(a, b)` is equal to either `a` or `b`, then
/// `bump_f64(result)` is returned, other the result is returned directly.
#[allow(dead_code)]
#[inline]
pub(crate) fn result_bumper_64(operation: fn(f64, f64) -> f64, a: f64, b: f64) -> f64 {
    let result = operation(a, b);
    if (result == a) || (result == b) {
        bump_f64(result)
    } else {
        result
    }
}

// returns the ascending root of a quadratic polynomial ax^2 + bx + c
pub fn quad_root_ascending(a: N64, b: N64, c: N64) -> Option<N64> {
    let determinant = b * b - a * c * n64(4.0);
    if determinant <= n64(0.0) {
        None
    } else if b >= n64(0.0) {
        Some((c * n64(2.0)) / (-b - determinant.sqrt()))
    } else {
        Some((-b + determinant.sqrt()) / (a * n64(2.0)))
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
            (quad_root_ascending(n64(1e-14), n64(2.0), n64(-1.0)).unwrap() - n64(0.5)).abs()
                < n64(1e-7)
        );
        assert!(
            (quad_root_ascending(n64(0.0), n64(2.0), n64(-1.0)).unwrap() - n64(0.5)).abs()
                < n64(1e-7)
        );
        assert!(
            (quad_root_ascending(n64(100.0), n64(-1.0), n64(-1e-16)).unwrap() - n64(0.01)).abs()
                < n64(1e-7)
        );
        assert!(quad_root_ascending(n64(0.0), n64(-2.0), n64(1.0))
            .unwrap()
            .is_infinite());
        assert!(quad_root_ascending(n64(-3.0), n64(0.0), n64(-1.0)).is_none());
        assert!(quad_root_ascending(n64(1.0), n64(1.0), n64(1.0)).is_none());
    }
}
