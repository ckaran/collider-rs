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

mod solvers;

use crate::geom::shape::PlacedBounds;
use crate::geom::*;
use num::BigRational;
use std::f64;

#[cfg(feature = "enable_serde")]
extern crate serde;
#[cfg(feature = "enable_serde")]
use self::serde::*;

// DurHitbox (and DurHbVel) is almost identical to Hitbox (and HbVel), except
// it uses a `duration` (amount of time until invalidation of the hitbox)
// rather than an `end_time` (time of the invalidation of the hitbox). This
// new struct is meant to make that distinction clear.

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
pub struct DurHbVel {
    pub value: Vec2,
    pub resize: Vec2,
    pub duration: BigRational,
}

impl DurHbVel {
    pub fn still() -> DurHbVel {
        DurHbVel {
            value: Vec2::zero(),
            resize: Vec2::zero(),
            duration: BigRational::from_float(f64::INFINITY).unwrap(),
        }
    }

    fn is_still(&self) -> bool {
        self.value == Vec2::zero() && self.resize == Vec2::zero()
    }

    fn negate(&self) -> DurHbVel {
        DurHbVel {
            value: -self.value,
            resize: -self.resize,
            duration: self.duration,
        }
    }
}

impl PlacedBounds for DurHbVel {
    fn bounds_center(&self) -> &Vec2 {
        &self.value
    }
    fn bounds_dims(&self) -> &Vec2 {
        &self.resize
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
pub struct DurHitbox {
    pub value: PlacedShape,
    pub vel: DurHbVel,
}

impl DurHitbox {
    pub fn new(value: PlacedShape) -> DurHitbox {
        DurHitbox {
            value,
            vel: DurHbVel::still(),
        }
    }

    pub fn advanced_shape(&self, time: BigRational) -> PlacedShape {
        self.value.advance(self.vel.value, self.vel.resize, time)
    }

    pub fn bounding_box(&self) -> PlacedShape {
        self.bounding_box_for(self.vel.duration)
    }

    pub fn bounding_box_for(&self, duration: BigRational) -> PlacedShape {
        if self.vel.is_still() {
            self.value.as_rect()
        } else {
            let end_value = self.advanced_shape(duration);
            self.value.bounding_box(&end_value)
        }
    }

    pub fn collide_time(&self, other: &DurHitbox) -> BigRational {
        solvers::collide_time(self, other)
    }

    pub fn separate_time(&self, other: &DurHitbox, padding: BigRational) -> BigRational {
        solvers::separate_time(self, other, padding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::dur_hitbox::DurHitbox;
    use std::f64;

    #[cfg(feature = "enable_serde")]
    use bincode::{deserialize, serialize};

    #[cfg(feature = "enable_serde")]
    #[test]
    fn test_serde_db_hb_vel() {
        let original = DurHbVel::still();

        let serialized = serialize(&original).unwrap();
        let duplicate: DurHbVel = deserialize(&serialized).unwrap();
        assert_eq!(original, duplicate);
    }

    #[cfg(feature = "enable_serde")]
    #[test]
    fn test_serde_dur_hitbox() {
        let mut original = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(-11.0).unwrap(),
                BigRational::from_float(0.0).unwrap(),
            ),
            Shape::rect(v2(
                BigRational::from_float(2.0).unwrap(),
                BigRational::from_float(2.0).unwrap(),
            )),
        ));
        original.vel.value = v2(
            BigRational::from_float(2.0).unwrap(),
            BigRational::from_float(0.0).unwrap(),
        );
        original.vel.duration = BigRational::from_float(100.0).unwrap();

        let serialized = serialize(&original).unwrap();
        let duplicate: DurHitbox = deserialize(&serialized).unwrap();
        assert_eq!(original, duplicate);
    }

    #[test]
    fn test_rect_rect_collision() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(-11.0).unwrap(),
                BigRational::from_float(0.0).unwrap(),
            ),
            Shape::rect(v2(
                BigRational::from_float(2.0).unwrap(),
                BigRational::from_float(2.0).unwrap(),
            )),
        ));
        a.vel.value = v2(
            BigRational::from_float(2.0).unwrap(),
            BigRational::from_float(0.0).unwrap(),
        );
        a.vel.duration = BigRational::from_float(100.0).unwrap();
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(12.0).unwrap(),
                BigRational::from_float(2.0).unwrap(),
            ),
            Shape::rect(v2(
                BigRational::from_float(2.0).unwrap(),
                BigRational::from_float(4.0).unwrap(),
            )),
        ));
        b.vel.value = v2(
            BigRational::from_float(-0.5).unwrap(),
            BigRational::from_float(0.0).unwrap(),
        );
        b.vel.resize = v2(
            BigRational::from_float(1.0).unwrap(),
            BigRational::from_float(0.0).unwrap(),
        );
        b.vel.duration = BigRational::from_float(100.0).unwrap();
        assert_eq!(a.collide_time(&b), BigRational::from_float(7.0).unwrap());
        assert_eq!(b.collide_time(&a), BigRational::from_float(7.0).unwrap());
        assert_eq!(
            a.separate_time(&b, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(0.0).unwrap()
        );
    }

    #[test]
    fn test_circle_circle_collision() {
        let sqrt2 = BigRational::from_float(2.0).unwrap().sqrt();
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(-0.1).unwrap() * sqrt2,
                BigRational::from_float(0.0).unwrap(),
            ),
            Shape::circle(BigRational::from_float(2.0).unwrap()),
        ));
        a.vel.value = v2(
            BigRational::from_float(0.1).unwrap(),
            BigRational::from_float(0.0).unwrap(),
        );
        a.vel.duration = BigRational::from_float(100.0).unwrap();
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(3.0).unwrap() * sqrt2,
                BigRational::from_float(0.0).unwrap(),
            ),
            Shape::circle(
                BigRational::from_float(2.0).unwrap()
                    + sqrt2 * BigRational::from_float(0.1).unwrap(),
            ),
        ));
        b.vel.value = v2(
            BigRational::from_float(-2.0).unwrap(),
            BigRational::from_float(1.0).unwrap(),
        );
        b.vel.resize = v2(
            BigRational::from_float(-0.1).unwrap(),
            BigRational::from_float(-0.1).unwrap(),
        );
        b.vel.duration = BigRational::from_float(100.0).unwrap();
        assert!((a.collide_time(&b) - sqrt2).abs() < BigRational::from_float(1e-7));
        assert_eq!(
            a.separate_time(&b, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(0.0).unwrap()
        );
    }

    #[test]
    fn test_rect_circle_collision() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(-11.0).unwrap(),
                BigRational::from_float(0.0).unwrap(),
            ),
            Shape::circle(BigRational::from_float(2.0).unwrap()),
        ));
        a.vel.value = v2(
            BigRational::from_float(2.0).unwrap(),
            BigRational::from_float(0.0).unwrap(),
        );
        a.vel.duration = BigRational::from_float(100.0).unwrap();
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(12.0).unwrap(),
                BigRational::from_float(2.0).unwrap(),
            ),
            Shape::rect(v2(
                BigRational::from_float(2.0).unwrap(),
                BigRational::from_float(4.0).unwrap(),
            )),
        ));
        b.vel.value = v2(
            BigRational::from_float(-1.0).unwrap(),
            BigRational::from_float(0.0).unwrap(),
        );
        b.vel.duration = BigRational::from_float(100.0).unwrap();
        assert_eq!(a.collide_time(&b), BigRational::from_float(7.0).unwrap());
        assert_eq!(b.collide_time(&a), BigRational::from_float(7.0).unwrap());
        assert_eq!(
            a.separate_time(&b, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(0.0).unwrap()
        );
    }

    #[test]
    fn test_rect_circle_angled_collision() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(0.).unwrap(),
                BigRational::from_float(0.).unwrap(),
            ),
            Shape::square(BigRational::from_float(2.).unwrap()),
        ));
        a.vel.duration = BigRational::from_float(100.0).unwrap();
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(5.).unwrap(),
                BigRational::from_float(5.).unwrap(),
            ),
            Shape::circle(BigRational::from_float(2.).unwrap()),
        ));
        b.vel.value = v2(
            BigRational::from_float(-1.).unwrap(),
            BigRational::from_float(-1.).unwrap(),
        );
        b.vel.duration = BigRational::from_float(100.0).unwrap();
        let collide_time = a.collide_time(&b);
        let expected_time = BigRational::from_float(4.).unwrap()
            - BigRational::from_float(1.).unwrap() / BigRational::from_float(2.0).unwrap().sqrt();
        assert_eq!(collide_time, expected_time);
    }

    #[test]
    fn test_rect_rect_separation() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(0.0).unwrap(),
                BigRational::from_float(0.0).unwrap(),
            ),
            Shape::rect(v2(
                BigRational::from_float(6.0).unwrap(),
                BigRational::from_float(4.0).unwrap(),
            )),
        ));
        a.vel.value = v2(
            BigRational::from_float(1.0).unwrap(),
            BigRational::from_float(1.0).unwrap(),
        );
        a.vel.duration = BigRational::from_float(100.0).unwrap();
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(1.0).unwrap(),
                BigRational::from_float(0.0).unwrap(),
            ),
            Shape::rect(v2(
                BigRational::from_float(4.0).unwrap(),
                BigRational::from_float(4.0).unwrap(),
            )),
        ));
        b.vel.value = v2(
            BigRational::from_float(0.5).unwrap(),
            BigRational::from_float(0.0).unwrap(),
        );
        b.vel.duration = BigRational::from_float(100.0).unwrap();
        assert_eq!(
            a.separate_time(&b, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(4.1).unwrap()
        );
        assert_eq!(
            b.separate_time(&a, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(4.1).unwrap()
        );
        assert_eq!(a.collide_time(&b), BigRational::from_float(0.0).unwrap());
    }

    #[test]
    fn test_circle_circle_separation() {
        let sqrt2 = BigRational::from_float(2.0).unwrap().sqrt();
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(2.0).unwrap(),
                BigRational::from_float(5.0).unwrap(),
            ),
            Shape::circle(BigRational::from_float(2.0).unwrap()),
        ));
        a.vel.duration = BigRational::from_float(100.0).unwrap();
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(3.0).unwrap(),
                BigRational::from_float(4.0).unwrap(),
            ),
            Shape::circle(BigRational::from_float(1.8).unwrap()),
        ));
        b.vel.value = v2(
            BigRational::from_float(-1.0).unwrap(),
            BigRational::from_float(1.0).unwrap(),
        );
        b.vel.duration = BigRational::from_float(100.0).unwrap();
        assert_eq!(
            a.separate_time(&b, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(1.0).unwrap() + sqrt2
        );
        assert_eq!(
            b.separate_time(&a, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(1.0).unwrap() + sqrt2
        );
        assert_eq!(a.collide_time(&b), BigRational::from_float(0.0).unwrap());
    }

    #[test]
    fn test_rect_circle_separation() {
        let sqrt2 = BigRational::from_float(2.0).unwrap().sqrt();
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(4.0).unwrap(),
                BigRational::from_float(2.0).unwrap(),
            ),
            Shape::rect(v2(
                BigRational::from_float(4.0).unwrap(),
                BigRational::from_float(6.0).unwrap(),
            )),
        ));
        a.vel.duration = BigRational::from_float(100.0).unwrap();
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(3.0).unwrap(),
                BigRational::from_float(4.0).unwrap(),
            ),
            Shape::circle(BigRational::from_float(3.8).unwrap()),
        ));
        b.vel.value = v2(
            BigRational::from_float(-1.0).unwrap(),
            BigRational::from_float(1.0).unwrap(),
        );
        b.vel.duration = BigRational::from_float(100.0).unwrap();
        assert_eq!(
            a.separate_time(&b, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(1.0).unwrap() + sqrt2
        );
        assert_eq!(
            b.separate_time(&a, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(1.0).unwrap() + sqrt2
        );
        assert_eq!(a.collide_time(&b), BigRational::from_float(0.0).unwrap());
    }

    #[test]
    fn test_rect_circle_angled_separation() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(0.).unwrap(),
                BigRational::from_float(0.).unwrap(),
            ),
            Shape::square(BigRational::from_float(2.).unwrap()),
        ));
        a.vel.duration = BigRational::from_float(100.0).unwrap();
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(-1.).unwrap(),
                BigRational::from_float(1.).unwrap(),
            ),
            Shape::circle(BigRational::from_float(2.).unwrap()),
        ));
        b.vel.value = v2(
            BigRational::from_float(1.).unwrap(),
            BigRational::from_float(-1.).unwrap(),
        );
        b.vel.duration = BigRational::from_float(100.0).unwrap();
        let separate_time = a.separate_time(&b, BigRational::from_float(0.1).unwrap());
        let expected_time = BigRational::from_float(2.).unwrap()
            + BigRational::from_float(1.1).unwrap() / BigRational::from_float(2.0).unwrap().sqrt();
        assert_eq!(separate_time, expected_time);
    }

    #[test]
    fn test_no_collision() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(-11.0).unwrap(),
                BigRational::from_float(0.0).unwrap(),
            ),
            Shape::rect(v2(
                BigRational::from_float(2.0).unwrap(),
                BigRational::from_float(2.0).unwrap(),
            )),
        ));
        a.vel.value = v2(
            BigRational::from_float(2.0).unwrap(),
            BigRational::from_float(0.0).unwrap(),
        );
        a.vel.duration = BigRational::from_float(100.0).unwrap();
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(12.0).unwrap(),
                BigRational::from_float(2.0).unwrap(),
            ),
            Shape::rect(v2(
                BigRational::from_float(2.0).unwrap(),
                BigRational::from_float(4.0).unwrap(),
            )),
        ));
        b.vel.value = v2(
            BigRational::from_float(-1.0).unwrap(),
            BigRational::from_float(1.0).unwrap(),
        );
        b.vel.duration = BigRational::from_float(100.0).unwrap();
        assert_eq!(
            a.collide_time(&b),
            BigRational::from_float(f64::INFINITY).unwrap()
        );
        assert_eq!(
            a.separate_time(&b, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(0.0).unwrap()
        );

        b.value.shape = Shape::circle(BigRational::from_float(2.0).unwrap());
        b.vel.resize = Vec2::zero();
        assert_eq!(
            a.collide_time(&b),
            BigRational::from_float(f64::INFINITY).unwrap()
        );
        assert_eq!(
            a.separate_time(&b, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(0.0).unwrap()
        );

        a.value.shape = Shape::circle(BigRational::from_float(2.0).unwrap());
        a.vel.resize = Vec2::zero();
        assert_eq!(
            a.collide_time(&b),
            BigRational::from_float(f64::INFINITY).unwrap()
        );
        assert_eq!(
            a.separate_time(&b, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(0.0).unwrap()
        );
    }

    #[test]
    fn test_no_separation() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(5.0).unwrap(),
                BigRational::from_float(1.0).unwrap(),
            ),
            Shape::rect(v2(
                BigRational::from_float(2.0).unwrap(),
                BigRational::from_float(2.0).unwrap(),
            )),
        ));
        a.vel.value = v2(
            BigRational::from_float(2.0).unwrap(),
            BigRational::from_float(1.0).unwrap(),
        );
        a.vel.duration = BigRational::from_float(100.0).unwrap();
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(5.0).unwrap(),
                BigRational::from_float(1.0).unwrap(),
            ),
            Shape::rect(v2(
                BigRational::from_float(2.0).unwrap(),
                BigRational::from_float(4.0).unwrap(),
            )),
        ));
        b.vel.value = v2(
            BigRational::from_float(2.0).unwrap(),
            BigRational::from_float(1.0).unwrap(),
        );
        b.vel.duration = BigRational::from_float(100.0).unwrap();
        assert_eq!(
            a.separate_time(&b, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(f64::INFINITY).unwrap()
        );
        assert_eq!(a.collide_time(&b), BigRational::from_float(0.0).unwrap());

        b.value.shape = Shape::circle(BigRational::from_float(2.0).unwrap());
        b.vel.resize = Vec2::zero();
        assert_eq!(
            a.separate_time(&b, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(f64::INFINITY).unwrap()
        );
        assert_eq!(a.collide_time(&b), BigRational::from_float(0.0).unwrap());

        a.value.shape = Shape::circle(BigRational::from_float(2.0).unwrap());
        a.vel.resize = Vec2::zero();
        assert_eq!(
            a.separate_time(&b, BigRational::from_float(0.1).unwrap()),
            BigRational::from_float(f64::INFINITY).unwrap()
        );
        assert_eq!(a.collide_time(&b), BigRational::from_float(0.0).unwrap());
    }

    #[test]
    fn test_low_duration() {
        let sqrt2 = BigRational::from_float(2.0).unwrap().sqrt();
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(0.0).unwrap(),
                BigRational::from_float(0.0).unwrap(),
            ),
            Shape::circle(BigRational::from_float(2.0).unwrap()),
        ));
        a.vel.duration =
            BigRational::from_float(4.0).unwrap() - sqrt2 + BigRational::from_float(0.01).unwrap();
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(
                BigRational::from_float(4.0).unwrap(),
                BigRational::from_float(4.0).unwrap(),
            ),
            Shape::circle(BigRational::from_float(2.0).unwrap()),
        ));
        b.vel.value = v2(
            BigRational::from_float(-1.0).unwrap(),
            BigRational::from_float(-1.0).unwrap(),
        );
        b.vel.duration =
            BigRational::from_float(4.0).unwrap() - sqrt2 + BigRational::from_float(0.01).unwrap();
        assert_eq!(
            a.collide_time(&b),
            BigRational::from_float(4.0).unwrap() - sqrt2
        );
        a.vel.duration -= BigRational::from_float(0.02).unwrap();
        assert_eq!(
            a.collide_time(&b),
            BigRational::from_float(f64::INFINITY).unwrap()
        );
        b.vel.duration -= BigRational::from_float(0.02).unwrap();
        assert_eq!(
            a.collide_time(&b),
            BigRational::from_float(f64::INFINITY).unwrap()
        );
    }
}
