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

use crate::core;
use crate::geom::shape::PlacedBounds;
use crate::geom::*;
use noisy_float::prelude::*;
use std::f64;

#[cfg(feature = "enable_serde")]
extern crate serde;
#[cfg(feature = "enable_serde")]
use self::serde::*;

// DurHitbox (and DurHbVel) is almost identical to Hitbox (and HbVel), except
// it uses a `duration` (amount of time until invalidation of the hitbox)
// rather than an `end_time` (time of the invalidation of the hitbox). This
// new struct is meant to make that distinction clear.

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
pub struct DurHbVel {
    pub value: Vec2,
    pub resize: Vec2,
    pub duration: N64,
}

impl DurHbVel {
    pub fn still() -> DurHbVel {
        DurHbVel {
            value: Vec2::zero(),
            resize: Vec2::zero(),
            duration: n64(f64::INFINITY),
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
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

    pub fn advanced_shape(&self, time: N64) -> PlacedShape {
        assert!(
            time < n64(core::HIGH_TIME),
            "requires time < {}",
            n64(core::HIGH_TIME)
        );
        self.value.advance(self.vel.value, self.vel.resize, time)
    }

    pub fn bounding_box(&self) -> PlacedShape {
        self.bounding_box_for(self.vel.duration)
    }

    pub fn bounding_box_for(&self, duration: N64) -> PlacedShape {
        if self.vel.is_still() {
            self.value.as_rect()
        } else {
            let end_value = self.advanced_shape(duration);
            self.value.bounding_box(&end_value)
        }
    }

    pub fn collide_time(&self, other: &DurHitbox) -> N64 {
        solvers::collide_time(self, other)
    }

    pub fn separate_time(&self, other: &DurHitbox, padding: N64) -> N64 {
        solvers::separate_time(self, other, padding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::dur_hitbox::DurHitbox;
    use std::f64;

    #[test]
    fn test_rect_rect_collision() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(n64(-11.0), n64(0.0)),
            Shape::rect(v2(n64(2.0), n64(2.0))),
        ));
        a.vel.value = v2(n64(2.0), n64(0.0));
        a.vel.duration = n64(100.0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(n64(12.0), n64(2.0)),
            Shape::rect(v2(n64(2.0), n64(4.0))),
        ));
        b.vel.value = v2(n64(-0.5), n64(0.0));
        b.vel.resize = v2(n64(1.0), n64(0.0));
        b.vel.duration = n64(100.0);
        assert_eq!(a.collide_time(&b), n64(7.0));
        assert_eq!(b.collide_time(&a), n64(7.0));
        assert_eq!(a.separate_time(&b, n64(0.1)), n64(0.0));
    }

    #[test]
    fn test_circle_circle_collision() {
        let sqrt2 = n64(2.0).sqrt();
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(n64(-0.1) * sqrt2, n64(0.0)),
            Shape::circle(n64(2.0)),
        ));
        a.vel.value = v2(n64(0.1), n64(0.0));
        a.vel.duration = n64(100.0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(n64(3.0) * sqrt2, n64(0.0)),
            Shape::circle(n64(2.0) + sqrt2 * n64(0.1)),
        ));
        b.vel.value = v2(n64(-2.0), n64(1.0));
        b.vel.resize = v2(n64(-0.1), n64(-0.1));
        b.vel.duration = n64(100.0);
        assert!((a.collide_time(&b) - sqrt2).abs() < n64(1e-7));
        assert_eq!(a.separate_time(&b, n64(0.1)), n64(0.0));
    }

    #[test]
    fn test_rect_circle_collision() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(n64(-11.0), n64(0.0)),
            Shape::circle(n64(2.0)),
        ));
        a.vel.value = v2(n64(2.0), n64(0.0));
        a.vel.duration = n64(100.0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(n64(12.0), n64(2.0)),
            Shape::rect(v2(n64(2.0), n64(4.0))),
        ));
        b.vel.value = v2(n64(-1.0), n64(0.0));
        b.vel.duration = n64(100.0);
        assert_eq!(a.collide_time(&b), n64(7.0));
        assert_eq!(b.collide_time(&a), n64(7.0));
        assert_eq!(a.separate_time(&b, n64(0.1)), n64(0.0));
    }

    #[test]
    fn test_rect_circle_angled_collision() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(n64(0.), n64(0.)),
            Shape::square(n64(2.)),
        ));
        a.vel.duration = n64(100.0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(n64(5.), n64(5.)),
            Shape::circle(n64(2.)),
        ));
        b.vel.value = v2(n64(-1.), n64(-1.));
        b.vel.duration = n64(100.0);
        let collide_time = a.collide_time(&b);
        let expected_time = n64(4.) - n64(1.) / n64(2.0).sqrt();
        assert_eq!(collide_time, expected_time);
    }

    #[test]
    fn test_rect_rect_separation() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(n64(0.0), n64(0.0)),
            Shape::rect(v2(n64(6.0), n64(4.0))),
        ));
        a.vel.value = v2(n64(1.0), n64(1.0));
        a.vel.duration = n64(100.0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(n64(1.0), n64(0.0)),
            Shape::rect(v2(n64(4.0), n64(4.0))),
        ));
        b.vel.value = v2(n64(0.5), n64(0.0));
        b.vel.duration = n64(100.0);
        assert_eq!(a.separate_time(&b, n64(0.1)), n64(4.1));
        assert_eq!(b.separate_time(&a, n64(0.1)), n64(4.1));
        assert_eq!(a.collide_time(&b), n64(0.0));
    }

    #[test]
    fn test_circle_circle_separation() {
        let sqrt2 = n64(2.0).sqrt();
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(n64(2.0), n64(5.0)),
            Shape::circle(n64(2.0)),
        ));
        a.vel.duration = n64(100.0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(n64(3.0), n64(4.0)),
            Shape::circle(n64(1.8)),
        ));
        b.vel.value = v2(n64(-1.0), n64(1.0));
        b.vel.duration = n64(100.0);
        assert_eq!(a.separate_time(&b, n64(0.1)), n64(1.0) + sqrt2);
        assert_eq!(b.separate_time(&a, n64(0.1)), n64(1.0) + sqrt2);
        assert_eq!(a.collide_time(&b), n64(0.0));
    }

    #[test]
    fn test_rect_circle_separation() {
        let sqrt2 = n64(2.0).sqrt();
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(n64(4.0), n64(2.0)),
            Shape::rect(v2(n64(4.0), n64(6.0))),
        ));
        a.vel.duration = n64(100.0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(n64(3.0), n64(4.0)),
            Shape::circle(n64(3.8)),
        ));
        b.vel.value = v2(n64(-1.0), n64(1.0));
        b.vel.duration = n64(100.0);
        assert_eq!(a.separate_time(&b, n64(0.1)), n64(1.0) + sqrt2);
        assert_eq!(b.separate_time(&a, n64(0.1)), n64(1.0) + sqrt2);
        assert_eq!(a.collide_time(&b), n64(0.0));
    }

    #[test]
    fn test_rect_circle_angled_separation() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(n64(0.), n64(0.)),
            Shape::square(n64(2.)),
        ));
        a.vel.duration = n64(100.0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(n64(-1.), n64(1.)),
            Shape::circle(n64(2.)),
        ));
        b.vel.value = v2(n64(1.), n64(-1.));
        b.vel.duration = n64(100.0);
        let separate_time = a.separate_time(&b, n64(0.1));
        let expected_time = n64(2.) + n64(1.1) / n64(2.0).sqrt();
        assert_eq!(separate_time, expected_time);
    }

    #[test]
    fn test_no_collision() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(n64(-11.0), n64(0.0)),
            Shape::rect(v2(n64(2.0), n64(2.0))),
        ));
        a.vel.value = v2(n64(2.0), n64(0.0));
        a.vel.duration = n64(100.0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(n64(12.0), n64(2.0)),
            Shape::rect(v2(n64(2.0), n64(4.0))),
        ));
        b.vel.value = v2(n64(-1.0), n64(1.0));
        b.vel.duration = n64(100.0);
        assert_eq!(a.collide_time(&b), n64(f64::INFINITY));
        assert_eq!(a.separate_time(&b, n64(0.1)), n64(0.0));

        b.value.shape = Shape::circle(n64(2.0));
        b.vel.resize = Vec2::zero();
        assert_eq!(a.collide_time(&b), n64(f64::INFINITY));
        assert_eq!(a.separate_time(&b, n64(0.1)), n64(0.0));

        a.value.shape = Shape::circle(n64(2.0));
        a.vel.resize = Vec2::zero();
        assert_eq!(a.collide_time(&b), n64(f64::INFINITY));
        assert_eq!(a.separate_time(&b, n64(0.1)), n64(0.0));
    }

    #[test]
    fn test_no_separation() {
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(n64(5.0), n64(1.0)),
            Shape::rect(v2(n64(2.0), n64(2.0))),
        ));
        a.vel.value = v2(n64(2.0), n64(1.0));
        a.vel.duration = n64(100.0);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(n64(5.0), n64(1.0)),
            Shape::rect(v2(n64(2.0), n64(4.0))),
        ));
        b.vel.value = v2(n64(2.0), n64(1.0));
        b.vel.duration = n64(100.0);
        assert_eq!(a.separate_time(&b, n64(0.1)), n64(f64::INFINITY));
        assert_eq!(a.collide_time(&b), n64(0.0));

        b.value.shape = Shape::circle(n64(2.0));
        b.vel.resize = Vec2::zero();
        assert_eq!(a.separate_time(&b, n64(0.1)), n64(f64::INFINITY));
        assert_eq!(a.collide_time(&b), n64(0.0));

        a.value.shape = Shape::circle(n64(2.0));
        a.vel.resize = Vec2::zero();
        assert_eq!(a.separate_time(&b, n64(0.1)), n64(f64::INFINITY));
        assert_eq!(a.collide_time(&b), n64(0.0));
    }

    #[test]
    fn test_low_duration() {
        let sqrt2 = n64(2.0).sqrt();
        let mut a = DurHitbox::new(PlacedShape::new(
            v2(n64(0.0), n64(0.0)),
            Shape::circle(n64(2.0)),
        ));
        a.vel.duration = n64(4.0) - sqrt2 + n64(0.01);
        let mut b = DurHitbox::new(PlacedShape::new(
            v2(n64(4.0), n64(4.0)),
            Shape::circle(n64(2.0)),
        ));
        b.vel.value = v2(n64(-1.0), n64(-1.0));
        b.vel.duration = n64(4.0) - sqrt2 + n64(0.01);
        assert_eq!(a.collide_time(&b), n64(4.0) - sqrt2);
        a.vel.duration -= n64(0.02);
        assert_eq!(a.collide_time(&b), n64(f64::INFINITY));
        b.vel.duration -= n64(0.02);
        assert_eq!(a.collide_time(&b), n64(f64::INFINITY));
    }
}
